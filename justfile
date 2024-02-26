sync_db:
  npx prisma db push --schema ./src/db/schema.prisma
  sea-orm-cli generate entity \
    -o src/db/entities

psql:
  PGPASSWORD=mysecretpassword psql -U postgres -d focus -h localhost
