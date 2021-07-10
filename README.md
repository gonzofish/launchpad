# Rocket Launchpad

This is my way of getting familiar with Rocket + Diesel.

I'm using [this tutorial](https://dev.to/pxjohnny/late-night-confessions-building-a-website-using-rust-rocket-diesel-and-askama-part-1-127j) as my basis, although I'm not creating the application in that tutorial.

For `rocket_sync_db_pools` I used [the database example from the Rocket repo](https://github.com/SergioBenitez/Rocket/tree/v0.5-rc/examples/databases) as a guide.

## Database Setup

Before running the application or running tests, you'll need databases to do so. This requires a user as well production & test databases to be created.

> In a real production environment, the database credentials would
> be more secure and injected from a secrets store

1. Setup the launchpad user

    ```
    createuser --interactive --pwprompt
    Enter name of role to add: launchpad
    Enter password for new role: <enter "launchpad">
    Enter it again: <enter "launchpad">
    Shall the new role be a superuser? (y/n) n
    Shall the new role be allowed to create databases? (y/n) n
    Shall the new role be allowed to create more new roles? (y/n) n
    ```
2. Set up test database

    ```
    > createdb -U <admin_user> launchpad_test
    > psql -U <admin_user> launchpad_test
    launchpad_test=# GRANT ALL PRIVILEGES ON DATABASE launchpad_test TO launchpad;
    GRANT
    launchpad_test=# \q
    ```
3. Set up "production" database

    ```
    > createdb -U <admin_user> launchpad_test
    > psql -U <admin_user> launchpad_test
    launchpad=# GRANT ALL PRIVILEGES ON DATABASE launchpad TO launchpad;
    GRANT
    launchpad=# \q
    ```
