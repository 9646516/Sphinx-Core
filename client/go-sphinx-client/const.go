package go_sphinx_client

var (
	PostSubmissionHeaderKeyProblemConfigPath = []byte("problem")
	PostSubmissionHeaderKeyLanguage          = []byte("lang")
	PostSubmissionHeaderKeySubmissionID      = []byte("uid")
)

const (
	Version = "v1001"
	ClientIdentifier = "go-sarama-" + Version
)