# TODO - Suggested Features

- [ ] Seed admin can invite more collaborators.
- [ ] The seeded admin user can be configured by an environment variable for deployment; maybe as a command line application.
- [ ] Add these validation checks to our `POST /admin/password` endpoint
- [ ] Use Postmark [batch email API](https://postmarkapp.com/developer/user-guide/send-email-with-api/batch-emails)
- [ ] Visibility into the delivery process - e.g. a page to track how many emails are still outstanding for a certain newsletter issue.
- [ ] When the delivery attempt fails due to a Postmark error try `n` times. This could be changed by enhancing `issue_delivery_queue` - e.g. adding a n_retries and execute_after columns to keep track of how many attempts have already taken place and how long we should wait before trying again.
- [ ] An expiry mechanism for our idempotency keys.
