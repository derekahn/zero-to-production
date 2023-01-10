# QA Instructions

This is a guide to perform QA both locally and on production.

## Database Migration

**Locally:**

When you execute `./scripts/init_db.sh` this will not only seed the database but handle the migrations as well.

**Production:**

It's always good to make sure the production database is up to date!

Run a quick migration on the **production** database:

```sh
DATABASE_URL="postgresql://newsletter:AVNS_fmPst2_xxxxxxxxxxxx@app-54b2e929-xxxx-xxxx-xxxx-xxxxxxxxxxxx-do-user-13110253-0.b.db.ondigitalocean.com:25060/newsletter" sqlx migrate run

  Applied 20221219044628/add status to subscriptions (239.6242ms)
  Applied 20221219044841/make status not null in subscriptions (321.333942ms)
  Applied 20221219045104/create subscription tokens table (141.623332ms)
  Applied 20221230223813/create users table (142.113942ms)
  Applied 20230101180748/rename password column (151.269324ms)
  Applied 20230101214920/add salt to users (104.52392ms)
  Applied 20230103211558/remove salt from users (122.222322ms)
  Applied 20230109000701/seed user (191.127712ms)
```

## Create A Subscriber:

**Locally:**

```sh
curl -X POST "http://localhost:8000/subscriptions" \
	-H "Content-Type: application/x-www-form-urlencoded" \
	-d "name=Satoshi%20Nakamoto&email=staoshi%2Bzero2prod%40bitcoin.com"
```

**Production:**

```sh
curl -X POST "https://zero2prod-xxxxx.ondigitalocean.app/subscriptions" \
	-H "Content-Type: application/x-www-form-urlencoded" \
	-d "name=Satoshi%20Nakamoto&email=staoshi%2Bzero2prod%40bitcoin.com"
```

## Confirm A Subscriber:

**Locally:**

```sh
curl "http://localhost:8000/subscriptions/confirm?subscription_token=EHRtc3MFJV8gmRydf2vmaK8YE"
```

**Production:**

```sh
curl "https://zero2prod-xxxxx.ondigitalocean.app/confirm?subscription_token=EHRtc3MFJV8gmRydf2vmaK8YE"
```

## Index

![Index View](../assets/qa/index.png)

## Admin Login

**If** it's a fresh database with the seeded user, the default password is: `everythinghastostartsomewhere`

**Login**

![login](../assets/qa/login.png)

**Admin Dashboard**

![admin dashboard](../assets/qa/admin_dashboard.png)

## Admin Password Reset

**Admin Password Change**

![admin password reset](../assets/qa/change_password.png)

**Successful Password Change**

![Successful password change](../assets/qa/changed_password.png)

## Send A Newsletter

**Publish A Newsletter**

![Publish newsletter](../assets/qa/publish_newsletter.png)

**Successful Published Newsletter**

![Published Newsletter](../assets/qa/published_newsletter.png)
