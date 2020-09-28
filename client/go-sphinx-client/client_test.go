package go_sphinx_client

import (
	"context"
	"fmt"
	"github.com/stretchr/testify/assert"
	"os"
	"testing"
)

// let cpp = read_to_string("../../test/a+b/Main.cpp").unwrap();
// buf.put("/home/rinne/Sphinx-Core/test/sb.toml");

func doPostSubmission(c *producerClient, t *testing.T) {
	t.Helper()

	code, err := os.Open("../../test/a+b/Main.cpp")
	assert.NoError(t, err)
	defer code.Close()

	res, err := c.PostSubmission(context.Background(), &SphinxPostSubmissionRequest{
		Code:              code,
		ProblemConfigPath: "/home/rinne/Sphinx-Core/test/sb.toml",
		Language:          1,
		SubmissionID:      0,
	})
	assert.NoError(t, err)
	fmt.Println(res)
}

func TestPostSubmission(t *testing.T) {
	cfg := &Config{
		BrokerHosts:                []string{"localhost:9092"},
		PostSubmissionRequestTopic: "in",
	}

	c, err := newProduceClient(cfg)
	assert.NoError(t, err)

	defer c.Close()

	doPostSubmission(&c, t)
}
type hhhh struct {

}

func (h hhhh) GetSubmissionResult(in *GetSubmissionResultRequest) (
	out *GetSubmissionResultReply, err error) {
	fmt.Println(in)
	return
}

func TestPostSubmissionNotify(t *testing.T) {
	cfg := &Config{
		BrokerHosts:                []string{"localhost:9092"},
		ConsumerGroup:              "Q2",
		PostSubmissionRequestTopic: "in",
		GetSubmissionResultTopic:   "result",
	}

	p, err := newProduceClient(cfg)
	assert.NoError(t, err)

	defer p.Close()

	doPostSubmission(&p, t)

	c, err := newConsumerClient(hhhh{}, cfg )
	assert.NoError(t, err)

	defer c.Close()
	err = c.Start(context.Background())
	assert.NotNil(t, err)
	assert.Equal(t, "context canceled", err.Error())
}
