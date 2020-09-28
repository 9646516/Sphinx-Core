package go_sphinx_client

import (
	"context"
	"encoding/binary"
	"errors"
	"fmt"
	"github.com/Shopify/sarama"
	"io/ioutil"
	"log"
	"os"
	"os/signal"
	"sync"
	"syscall"
)

type consumerClient struct {
	consumerGroup sarama.ConsumerGroup
	config *Config

	exitErr error
	setUpNotify chan bool

	handler SphinxEventHandler
}

func newConsumerClient(handler SphinxEventHandler, cfg *Config) (c consumerClient, err error) {
	c = consumerClient{config: cfg}
	c.consumerGroup, err = c.config.NewConsumerGroup()
	c.handler = handler
	return
}

func (c *consumerClient) Setup(sarama.ConsumerGroupSession) error {
	close(c.setUpNotify)
	return nil
}

func (c *consumerClient) Cleanup(sarama.ConsumerGroupSession) error {
	return nil
}

func (c *consumerClient) ConsumeClaim(
	ses sarama.ConsumerGroupSession, claim sarama.ConsumerGroupClaim) error {
	for message := range claim.Messages() {
		switch message.Topic {
		case c.config.GetSubmissionResultTopic:
			var request GetSubmissionResultRequest
			var ok bool
			var err error
			request.JudgeStatus, ok = judgeStatusMapping[string(message.Value)]
			if !ok {
				request.JudgeStatus = JudgeStatusUnknownError
			}

			if len(message.Headers) != 6 {
				ses.MarkMessage(message, "header error")
				continue
			}

			if len(message.Headers[0].Value) +
				len(message.Headers[1].Value) +
				len(message.Headers[2].Value) +
				len(message.Headers[3].Value) +
				len(message.Headers[4].Value) != 36 {
				fmt.Println("gg", len(message.Headers[0].Value) +
					len(message.Headers[1].Value) +
					len(message.Headers[2].Value) +
					len(message.Headers[3].Value) +
					len(message.Headers[4].Value))
				for _, h := range message.Headers {
					fmt.Println(h.Key, h.Value)
				}
				continue
			}

			request.Key = message.Key
			request.Mem = binary.BigEndian.Uint64(message.Headers[0].Value)
			request.Time = binary.BigEndian.Uint64(message.Headers[1].Value)
			request.SubmissionID = binary.BigEndian.Uint64(message.Headers[2].Value)
			request.Last = binary.BigEndian.Uint32(message.Headers[3].Value)
			request.Score = binary.BigEndian.Uint64(message.Headers[4].Value)
			request.Info = message.Headers[5].Value

			_, err = c.handler.GetSubmissionResult(&request)
			if err != nil {
				log.Println(err)
			} else {
				ses.MarkMessage(message, "")
			}
		default:
			log.Printf("%v", errors.New("unknown topic"))
		}
	}

	return nil
}

func (c *consumerClient) Start(ctx context.Context) error {
	c.setUpNotify = make(chan bool)
	var wg sync.WaitGroup
	wg.Add(1)

	subCtx, cancel := context.WithCancel(ctx)

	go func() {
		defer wg.Done()
		for {

			err := c.consumerGroup.Consume(subCtx, []string{
				c.config.GetSubmissionResultTopic,
			}, c)

			if err != nil {
				c.exitErr = err
			}

			if subCtx.Err() != nil {
				c.exitErr = subCtx.Err()
				return
			}

			c.setUpNotify = make(chan bool)
		}
	}()

	<-c.setUpNotify

	// todo: move out
	sigterm := make(chan os.Signal, 1)
	signal.Notify(sigterm, syscall.SIGINT, syscall.SIGTERM)
	select {
	case <-ctx.Done():
		fmt.Println("...")
		break
	case <-sigterm:
		fmt.Println("......")
		break
	}

	cancel()
	wg.Wait()
	return c.exitErr
}

func (c *consumerClient) Close() error {
	return c.consumerGroup.Close()
}

type producerClient struct {
	producer sarama.SyncProducer
	config   *Config
}

func newProduceClient(cfg *Config) (c producerClient, err error) {
	c = producerClient{config: cfg}
	c.producer, err = c.config.NewProducerSync()
	return
}

func (c *producerClient) getU64B() []byte {
	return make([]byte, 8)
}

func (c *producerClient) putU64B(_ []byte) {
}

func (c *producerClient) Close() error {
	return c.producer.Close()
}

func (c *producerClient) PostSubmission(
	ctx context.Context, in *SphinxPostSubmissionRequest) (
	out *SphinxPostSubmissionReply, err error) {
	b, err := ioutil.ReadAll(in.Code)
	if err != nil {
		return nil, err
	}

	select {
	case <-ctx.Done():
		return nil, ctx.Err()
	default:
	}

	var lang, uid = c.getU64B(), c.getU64B()
	binary.BigEndian.PutUint64(lang, in.Language)
	binary.BigEndian.PutUint64(uid, in.SubmissionID)
	msg := &sarama.ProducerMessage{
		Topic: c.config.PostSubmissionRequestTopic,
		Headers: []sarama.RecordHeader{
			{Key: PostSubmissionHeaderKeyProblemConfigPath, Value: []byte(in.ProblemConfigPath)},
			{Key: PostSubmissionHeaderKeyLanguage, Value: lang},
			{Key: PostSubmissionHeaderKeySubmissionID, Value: uid},
		},
		Key:   sarama.StringEncoder("233"),
		Value: sarama.ByteEncoder(b),
	}

	c.putU64B(lang)
	c.putU64B(uid)

	out = new(SphinxPostSubmissionReply)
	out.Partition, out.Offset, err = c.producer.SendMessage(msg)
	return
}
