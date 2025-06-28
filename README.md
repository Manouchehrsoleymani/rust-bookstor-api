# rust-bookstor-api


In this Project, we'll create a REST API server in Rust to represent a bookstore. This bookstore would allow us to add books and authors.


Using this system, we'll be able to:

    - Create, list, update and delete authors

    - Create, list, update and delete books

    - Associate and disassociate books and authors

    - List all books by a particular author

## How to I do that:

- Create a REST API server using Rocket
- Interacting with a database using SeaORM
- Authenticating APIs using JWT
- Writing database migrations using SeaORM CLI
- Routing and HTTP methods
- Extracting data from HTTP requests
- Interacting with the database to query and insert data
- Creating user accounts
- Authentication using JWT
- Creating and using relationships between models to query and list associated data
- Handle incoming (request) and outgoing (response) data in a type safe way
- Handle CORS
- Create and run database migrations
- Create one-to-many database relationships
- Create entities from database tables

### I'll be using the following crates:

    - Rocket:a web framework for Rust that makes it simple to write fast, secure web applications without sacrificing flexibility, usability, or type safety.

    - SeaORM:is a relational ORM to help you build web services in Rust.

    - jsonwebtokento create and decode JWTs in a strongly typed way.

    - serde_jsonfor serializing and deserializing Rust data structures efficiently and generically.

I'm going to Dockerize our API server to deploy and run it anywhere.

## How To run :
```
- cargo install sea-orm-cli
- sea migrate init -d ./src/migrator/
- remove all unneccessary files and folders from migrate directory [rename lib.rs to mod.rm and move mod.rs and XXXX_creat_table.rs to migrate folder and remove all others]
- develop the tables Structure into table files
- sea migrate generate -d ./src/migrator create_author_table
- sea migrate generate -d ./src/migrator create_book_table
- # Generate entity files of database `bookstore` to `src/entities` run into root of project
$ sea-orm-cli generate entity \
    -u mysql://root:my_secret@localhost:3306/bookstore \
    -o src/entities


```