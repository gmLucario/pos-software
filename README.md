# About

Help to manage a small store like:
- Handle your inventory
- Make sales
- Sales info
- Products to buy
...


# Todo

- [x] select migration tool
- [x] select dependency make interaction db
- [x] define basic models
- [x] data - sales_info view linked
- [x] read input external device
- [ ] catalog flow
    - [x] schemas
    - [x] save new products list to save
    - [x] delete product record before save them 
    - [x] save list products
    - [ ] show message products saved/not saved
- [ ] view edit catalog
- [ ] sale flow
    - [x] validate amount product input
    - [x] btns `ok` and `cancel` form product
    - [x] list products view
    - [x] logic remove product list
    - [x] total to pay sale view
    - [x] payback money logic
    - [x] store operations
    - [x] update tables based on operations
    - [x] save info if sale is a loan
    - [ ] show message products can't be added to sale list
- [ ] sale loan info
    - [ ] query get paginated loans by `name_debtor` and `range_dates`
    - [ ] view search loans
- [ ] sale info statics
    - [ ] sale earnings and total list
- [ ] logger handler
    - [ ] save logs file
- [ ] add general doc comments
    - [ ] models module docs
    - [x] schemas module docs
    - [x] views module docs
- [ ] add unit tests
- [ ] add integration tests
- [ ] rollback when db errors


# For devs

## Run locally the app

1. start docker compose

```bash
$ docker compose up -d 
```

2. Run app

```
$ cargo run
```

## Migrations

Install [sqlx-cli](https://crates.io/crates/sqlx-cli) to run the migrations:

```bash
$ sqlx migrate add -r <name>
```

```bash
$ sqlx migrate run
```

```bash
$ sqlx migrate revert
```

## Define enviroment variables

You can use [direnv](https://direnv.net/) to manage them or create a `.env`

Using [direnv](https://direnv.net/)

1. Create a `.envrc`
2. Define the varibles in `.env.default`
3. Run

```bash
$ direnv allow .
```

## Open docs

```bash
$ cargo doc --no-deps --open
```

# Useful links
1. [frontend-example-iced](https://github.com/zupzup/rust-frontend-example-iced/blob/main/src/main.rs)
2. [iced-application-impl](https://github.com/irvingfisica/iced_examples/blob/master/examples/hola_app.rs)
