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

func TestPostSubmission(t *testing.T) {
	cfg := &Config{
		BrokerHosts:                []string{"localhost:9092"},
		PostSubmissionRequestTopic: "in",
	}

	c, err := newClient(cfg)
	assert.NoError(t, err)

	defer c.Close()

	code, err := os.Open("../../test/a+b/Main.cpp")
	assert.NoError(t, err)
	defer code.Close()

	res, err := c.PostSubmissionRequest(context.Background(), &SphinxPostSubmissionRequest{
		Code:              code,
		ProblemConfigPath: "/home/rinne/Sphinx-Core/test/sb.toml",
		Language:          1,
		SubmissionID:      0,
	})
	assert.NoError(t, err)
	fmt.Println(res)
}
