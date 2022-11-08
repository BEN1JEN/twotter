# Twotter

Like Twitter, but less musky.

## Architecture
This is just a rough plan for now.

- Data Storage (SurrealDB?)
	- User
		- Tag
      - Display name
		- Salted password hash
		- Active tokens
		- Icon
      - Privilege level (for now just normal or admin)
      - Pronouns
      - Bio
      - Liked posts
      - Following
   - File (Stored in db based on hash and also stored on disk)
     - Uploader
     - File name
     - Data type (for now just image, video or audio?)
   - Post (called a twot?)
     - Poster
     - Content
     - Attached file references
     - Retwot reference(s?)
     - User @s
     - Hashtags
     - Comments?
- Server requests (Rust+Actix Web?)
  - Unauthenticated
    - Create user
    - Login
  - Authenticated
    - Edit user info
    - Change user password
    - Post new twot
    - Like twot
    - Follow user
    - Get latest twots from followers
  - Authentication-agnostic
    - Get latest twots
      - Potentially add reddit-like "hot" ranking algorithm?
    - Get latest twot by user
    - Get latest twots with a tag
    - Search twots?
    - Search users?
- Website pages (Typescript+React or Rust+Yew?)
  - Home page
  - User page
  - Twot page? Or should this just be a pop-up?
  - Search page?
