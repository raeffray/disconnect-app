
# Disconnect Social Media



### Backing Service:
```
docker compose up -d
```

Installing Diesel CLI

```
cargo install diesel_cli --no-default-features --features postgres
```

Maybe need: 
```
brew install libpq
brew link --force libpq
echo 'export PATH="/opt/homebrew/opt/libpq/bin:$PATH"' >> ~/.zshrc

```


Setup Diesel
```
diesel setup --database-url postgres://disconnect-app:qwaszx12@localhost:5432/disconnect-app-storage

```

Creating the migration files

```
diesel migration generate create_memberships
```
```
diesel migration generate create_fellows
```
```
diesel migration run  --database-url postgres://disconnect-app:qwaszx12@localhost:5432/disconnect-app-storage
```