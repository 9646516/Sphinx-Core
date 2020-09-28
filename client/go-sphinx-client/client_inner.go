package go_sphinx_client

import (
	"context"
	"encoding/binary"
	"github.com/Shopify/sarama"
	"io/ioutil"
)

type client struct {
	producer sarama.SyncProducer
	config   *Config
}

func newClient(cfg *Config) (c client, err error) {
	c = client{config: cfg}
	c.producer, err = c.config.NewProducerSync()
	return
}

func (c *client) getU64B() []byte {
	return make([]byte, 8)
}

func (c *client) putU64B(_ []byte) {
}

func (c *client) Close() error {
	return c.producer.Close()
}

func (c *client) PostSubmissionRequest(
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
