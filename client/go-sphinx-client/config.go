package go_sphinx_client

import "github.com/Shopify/sarama"

type Config struct {
	BrokerHosts   []string
	ProducerGroup string

	PostSubmissionRequestTopic string
}

func (cfg *Config) NewConsumer() (sarama.Consumer, error) {
	return sarama.NewConsumer(cfg.BrokerHosts, nil)
}

//let producer: FutureProducer = ClientConfig::new()
//.set("bootstrap.servers", brokers)
//.set("produce.offset.report", "true")
//.set("message.timeout.ms", "5000")
//.create()
//.expect("Producer creation error");

func (cfg *Config) NewProducerSync() (sarama.SyncProducer, error) {
	config := sarama.NewConfig() // 1
	config.Producer.Return.Successes = true
	config.Producer.Return.Errors = true
	config.Producer.Partitioner = sarama.NewRandomPartitioner
	config.Version = sarama.V2_6_0_0

	return sarama.NewSyncProducer(cfg.BrokerHosts, config)
}
