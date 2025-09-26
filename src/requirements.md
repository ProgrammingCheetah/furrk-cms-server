# Requirements - Bot MVP

## Sections

### Root

`User`s should be able to:

- `Ping` the server to check if it is alive

### Authorization

`User`s should be able to:

- `Login`—specifically, be able to grab a set of Telegram values and send it to the backend, coupling them tightly.
- `Authenticate`—To prove that they are who they are, through the login
- `Authorize`—The backend should be able to manage permissions. Active Directory?
  This assumes that all of these use a JWT

### Posts

`Mod`s should be able to:

- `Create a Post`
- `Remove a Post`
- `Check the Queue`
- `Remove Posts from the Queue`
- `Check the Tags`

### Management
`Admin`s should be able to:
- `Manage Mods`
- `Everything Above This`