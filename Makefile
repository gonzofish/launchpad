.PHONY: run test

create-dev-db:
	docker run -d -p 5433:5432 --name launchpad_db -e POSTGRES_USER=launchpad -e POSTGRES_PASSWORD=launchpad -e POSTGRES_DB=launchpad postgres:13-alpine

create-test-db:
	docker run -d -p 5433:5432 --name launchpad_test_db -e POSTGRES_USER=launchpad -e POSTGRES_PASSWORD=launchpad -e POSTGRES_DB=launchpad_test postgres:13-alpine

destroy-dev-db:
	docker container stop launchpad_db
	docker container rm launchpad_db

destroy-test-db:
	docker container stop launchpad_test_db
	docker container rm launchpad_test_db

migrate: # Run migrations
	ln -sf .env.devel .env
	diesel migration run

run: # Run application
	${MAKE} setup-dev-db
	-cargo run
	${MAKE} teardown-dev-db

setup-dev-db:
	ln -sf .env.devel .env
	${MAKE} create-dev-db
	${MAKE} wait-for-dev-db
	diesel migration run

setup-test-db: # Set up DB for tests
	ln -sf .env.test .env
	${MAKE} create-test-db
	${MAKE} wait-for-test-db
	diesel migration run

teardown-dev-db: # Tear down test database
	${MAKE} destroy-dev-db
	ln -sf .env.devel .env

teardown-test-db: # Tear down test database
	${MAKE} destroy-test-db
	ln -sf .env.devel .env

test:  # Run tests with DB setup/teardown
	${MAKE} setup-test-db
	-cargo test
	${MAKE} teardown-test-db

wait-for-dev-db:
	docker exec -it launchpad_db sh -c 'until $$(nc -z localhost 5432); do { printf '.'; sleep 1; }; done; echo ""'

wait-for-test-db:
	docker exec -it launchpad_test_db sh -c 'until $$(nc -z localhost 5432); do { printf '.'; sleep 1; }; done; echo ""'
