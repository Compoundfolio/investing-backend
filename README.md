
### Running locally
To run the backend infrastructure locally, run the following commands

```sh
systemctl start docker
DATABASE_VOLUME=/tmp/pgdata docker-compose up

```
When running locally, server won't set up the database schema, you are expected to do that yourself:
```sh
diesel migration run
```
Now the backend iteself is ready to be run. In a separate terminal, run the following
```sh
cargo run
```
In order to interact with the API, you will need an auth token. Login-password auth is available when
running locally by address http://localhost:8080/static/auth.html

After authorizing, you can visit the graphql playground: 


### Uploading report
After adding the bearer token into the graphql playground, make the following graphql mutation
```graphql
mutation {
  createPortfolio(data: {label: "My new portfolio"}) {
    id, label
  }
}
```
Store the ID of the generated report. Then, you will have to execute a command in terminal:
```sh
curl -w "\nTotal time: %{time_total}s\n" --show-error -D -
    -X POST localhost:8080/graphql 
    -H "Authorization: Bearer $TOKEN" 
    --form 'operations={"query": "mutation UploadFile($file: Upload!) { uploadReport(brokerage: EXANTE, portfolioId: \"d5bd66bb-d8fb-4da2-849e-5af7593a35ba\", upload: $file) { id, transactions, tradeOperations} }", "variables": { "file": null } }' 
    --form 'map={ "nFile": ["variables.file"] }' 
    --form nFile=@testdata/exante_small_report.csv
```
