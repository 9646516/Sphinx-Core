all:
	docker build . --tag judge:1.0.0
	docker create --interactive -v /home/rinne/code:/code --name XJB judge:1.0.0
	docker start XJB

clean:
	docker rm -f $$(docker ps -aq)

.PHONY: all

.PHONY: clean


