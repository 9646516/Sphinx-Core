run:
	cargo fmt
	cargo run --release

image:
	docker build . --tag judge:1.0.0

clean:
	docker rm -f $$(docker ps -aq)

all:
	rm -rf /home/rinne/code
	mkdir /home/rinne/code
	rm -rf /home/rinne/data
	mkdir /home/rinne/data
	sudo chmod -R 777 /home/rinne/code
	docker create --interactive -v /home/rinne/code:/code \
	-v /home/rinne/data:/data --name XJB --tty --cpu-quota 100000 \
	--cpu-period 100000 --network none judge:1.0.0
	docker start XJB
	make core
	mkdir /home/rinne/data/a+b
	make test

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

test:
	cargo test --release -- --nocapture

.PHONY: run
.PHONY: all
.PHONY: image
.PHONY: clean
.PHONY: test
.PHONY: core
.PHONY: test
