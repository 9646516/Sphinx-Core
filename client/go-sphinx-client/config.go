package go_sphinx_client

import (
	"github.com/Shopify/sarama"
	"time"
	"github.com/google/uuid"
)

type Config struct {
	BrokerHosts   []string
	ProducerGroup string
	ConsumerGroup string

	PostSubmissionRequestTopic string
	GetSubmissionResultTopic string
}

func (cfg *Config) SaramaCommonConfig() (config *sarama.Config) {
	config = sarama.NewConfig()
	config.Net.ReadTimeout = time.Second * 5
	config.Net.WriteTimeout = time.Second * 5

	config.ClientID = ClientIdentifier + "-" + uuid.New().String()
	config.Version = sarama.V2_6_0_0
	return
}

func (cfg *Config) NewConsumerGroup() (sarama.ConsumerGroup, error) {
	config := cfg.SaramaCommonConfig()

	return sarama.NewConsumerGroup(cfg.BrokerHosts, cfg.ConsumerGroup, config)
}

//.set("produce.offset.report", "true")

func (cfg *Config) NewProducerSync() (sarama.SyncProducer, error) {
	config := cfg.SaramaCommonConfig()
	config.Producer.Return.Successes = true
	config.Producer.Return.Errors = true
	config.Producer.Partitioner = sarama.NewRandomPartitioner

	return sarama.NewSyncProducer(cfg.BrokerHosts, config)
}
