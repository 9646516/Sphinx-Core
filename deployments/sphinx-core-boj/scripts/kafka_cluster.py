import argparse
import os
import re
import sys

config = {
    'kafka_data': '/home/rinne/data/kafka-data',
    'zookeeper_port': 2181,
    'kafka_port': 9092,
    'kafka_container_name': 'sphinx-core-boj_kafka_1'
}


def set_args(f):
    def wrap(args=None):
        if args is None:
            args = sys.argv[1:]
        return f(args)

    return wrap


def format_config(content):
    for k, v in config.items():
        content = content.replace(f'{{{{{k}}}}}', str(v))
    matched = list(set(re.findall(r'\{\{([^\s}]*)}}', content)))
    if len(matched) != 0:
        raise KeyError(f"not enough variables, please provide variable in {matched}")
    return content


@set_args
def generate(args):
    parser = argparse.ArgumentParser()
    parser.add_argument("--src", type=str, required=True, help="source")
    parser.add_argument("--dst", type=str, required=True, help="target")
    args = parser.parse_args(args)

    if not os.path.exists(args.src):
        raise LookupError(f"source {args.src} not found")

    content = open(args.src).read()
    open(args.dst, 'w').write(format_config(content))


kafka_topics_executable = """docker exec {{kafka_container_name}} kafka-topics"""
zookeeper_option = """--zookeeper zookeeper:{{zookeeper_port}}"""


def invoke_kafka_topics(options):
    command_template = ' '.join([kafka_topics_executable, zookeeper_option] + options)
    cmd = format_config(command_template)

    print(cmd)
    import subprocess
    process = subprocess.Popen(cmd, shell=True, stdout=subprocess.PIPE)
    process.wait()
    return process.returncode


def insert_topic(args):
    parser = argparse.ArgumentParser()
    args = parser.parse_args(args)

    invoke_kafka_topics(["233"])


def list_topic(_=None):
    invoke_kafka_topics(["--list"])


def clean_topics(_=None):
    code = invoke_kafka_topics([" --delete --topic in"])
    code = code and invoke_kafka_topics([" --delete --topic result"])
    return code


commands = {
    'generate': generate,
    'insert_topic': insert_topic,
    'list_topic': list_topic,
    'clean_topics': clean_topics,
}

if __name__ == '__main__':
    command, raw_args = sys.argv[1], sys.argv[2:]

    if command not in commands:
        raise KeyError(f"command not found, got {command}")

    commands[command](sys.argv[2:])
