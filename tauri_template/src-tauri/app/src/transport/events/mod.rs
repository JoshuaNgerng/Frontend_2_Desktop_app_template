
/*
Characteristics
One-to-many communication
No return value
Used for async notifications
Decoupled from caller
Typical uses:
“Download finished”
“User logged in”
“Sync completed”
“DB updated”
Mental model:

“Something happened → notify UI”
*/