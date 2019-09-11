image:
	docker build . --tag judge:1.0.0

clean:
	docker rm -f $$(docker ps -aq)

test:
	rm -rf /home/rinne/data/a+b
	mkdir /home/rinne/data/a+b
	echo "1 2" > /home/rinne/data/a+b/1.in
	echo "3" > /home/rinne/data/a+b/1.out
	echo "10 20" > /home/rinne/data/a+b/2.in
	echo "30" > /home/rinne/data/a+b/2.out
	echo "100 200" > /home/rinne/data/a+b/3.in

core:
	gcc Core.c -o /home/rinne/code/core -lpthread -O2 -Wall
	g++ Jury.cpp -o /home/rinne/code/Jury -O2 -Wall -std=c++17

RunTest:
	cargo test --release -- --nocapture

RunZoo:
	cd ~/桌面/kafka_2.12-2.3.0 && \
	bin/zookeeper-server-start.sh config/zookeeper.properties

RunKafka:
	cd ~/桌面/kafka_2.12-2.3.0 && \
	bin/kafka-server-start.sh config/server.properties
	cd ~/桌面/kafka_2.12-2.3.0 && \
	bin/kafka-topics.sh --create --zookeeper localhost:2181 --replication-factor 1 --partitions 1 --topic in
	cd ~/桌面/kafka_2.12-2.3.0 && \
	bin/kafka-topics.sh --create --zookeeper localhost:2181 --replication-factor 1 --partitions 1 --topic result

StopZoo:
	cd ~/桌面/kafka_2.12-2.3.0 && \
	bin/zookeeper-server-stop.sh

list:
	cd ~/桌面/kafka_2.12-2.3.0 && \
	bin/kafka-topics.sh --list --zookeeper localhost:2181
	cd ~/桌面/kafka_2.12-2.3.0 && \
	bin/kafka-topics.sh --describe --zookeeper localhost:2181 --topic in
	cd ~/桌面/kafka_2.12-2.3.0 && \
	bin/kafka-topics.sh --describe --zookeeper localhost:2181 --topic result

.PHONY: image
.PHONY: clean
.PHONY: test
.PHONY: core
.PHONY: RunTest
.PHONY: RunZoo
.PHONY: RunKafka
.PHONY: StopZoo
.PHONY: list