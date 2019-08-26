deploy:
	rm -rf /home/rinne/code
	mkdir /home/rinne/code
	rm -rf /home/rinne/data
	mkdir /home/rinne/data
	sudo chmod -R 777 /home/rinne/code
	docker build . --tag judge:1.0.0
	docker create --interactive -v /home/rinne/code:/code -v /home/rinne/data:/data --name XJB judge:1.0.0
	docker start XJB

clean:
	docker rm -f $$(docker ps -aq)

.PHONY: deploy

.PHONY: clean


