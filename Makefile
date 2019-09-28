image:
	docker build . --tag judge:1.0.0

clean:
	docker rm -f $$(docker ps -aq)

build:
	mkdir -p /home/rinne/Spinex/data/1
	mkdir -p /home/rinne/Spinex/data/2
	mkdir -p /home/rinne/Spinex/code
	mkdir -p /home/rinne/Spinex/checker
	mkdir -p /home/rinne/Spinex/core
	echo "1 2" > /home/rinne/Spinex/data/1/1.in
	echo "3" > /home/rinne/Spinex/data/1/1.out
	echo "10 20" > /home/rinne/Spinex/data/1/2.in
	echo "30" > /home/rinne/Spinex/data/1/2.out
	echo "100 200" > /home/rinne/Spinex/data/1/3.in
	echo "300" > /home/rinne/Spinex/data/1/3.out
	echo "300" > /home/rinne/Spinex/data/2/1.in
	echo "500" > /home/rinne/Spinex/data/2/2.in
	echo "700" > /home/rinne/Spinex/data/2/3.in
	sudo chmod -R 770 /home/rinne/Spinex/data/
	gcc Core.c -o /home/rinne/Spinex/core/core -lpthread -O2 -Wall
	gcc Core2.c -o /home/rinne/Spinex/core/core2 -lpthread -O2 -Wall
	g++ Jury.cpp -o /home/rinne/Spinex/checker/Jury -O2 -Wall -std=c++17
	g++ test/binary_search/judge.cpp -o /home/rinne/Spinex/checker/2 -O2 -Wall -std=c++17

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
.PHONY: build
.PHONY: RunTest
.PHONY: RunZoo
.PHONY: RunKafka
.PHONY: StopZoo
.PHONY: list