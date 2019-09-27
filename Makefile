image:
	docker build . --tag judge:1.0.0

clean:
	docker rm -f $$(docker ps -aq)

dir:
	mkdir /home/rinne/data/
	mkdir /home/rinne/data/a+b
	mkdir /home/rinne/code
test:
	echo "1 2" > /home/rinne/data/a+b/1.in
	echo "3" > /home/rinne/data/a+b/1.out
	echo "10 20" > /home/rinne/data/a+b/2.in
	echo "30" > /home/rinne/data/a+b/2.out
	echo "100 200" > /home/rinne/data/a+b/3.in
	echo "300" > /home/rinne/data/a+b/3.out

core:
	gcc Core.c -o /home/rinne/code/core -lpthread -O2 -Wall
	gcc Core2.c -o /home/rinne/code/core2 -lpthread -O2 -Wall
	g++ Jury.cpp -o /home/rinne/code/Jury -O2 -Wall -std=c++17

RunTest:
	cargo test --release -- --nocapture

RunZoo:
	cd ~/kafka_2.12-2.3.0 && \
	bin/zookeeper-server-start.sh config/zookeeper.properties

RunKafka:
	cd ~/kafka_2.12-2.3.0 && \
	bin/kafka-server-start.sh config/server.properties
	cd ~/kafka_2.12-2.3.0 && \
	bin/kafka-topics.sh --create --zookeeper localhost:2181 --replication-factor 1 --partitions 1 --topic in
	cd ~/kafka_2.12-2.3.0 && \
	bin/kafka-topics.sh --create --zookeeper localhost:2181 --replication-factor 1 --partitions 1 --topic result

StopZoo:
	cd ~/kafka_2.12-2.3.0 && \
	bin/zookeeper-server-stop.sh

list:
	cd ~/kafka_2.12-2.3.0 && \
	bin/kafka-topics.sh --list --zookeeper localhost:2181
	cd ~/kafka_2.12-2.3.0 && \
	bin/kafka-topics.sh --describe --zookeeper localhost:2181 --topic in
	cd ~/kafka_2.12-2.3.0 && \
	bin/kafka-topics.sh --describe --zookeeper localhost:2181 --topic result

.PHONY: image
.PHONY: clean
.PHONY: test
.PHONY: core
.PHONY: dir
.PHONY: RunTest
.PHONY: RunZoo
.PHONY: RunKafka
.PHONY: StopZoo
.PHONY: list