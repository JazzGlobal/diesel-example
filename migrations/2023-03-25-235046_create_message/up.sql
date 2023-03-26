CREATE TABLE Messages (
    message_id int PRIMARY KEY,
    text varchar(255) NOT NULL,
    user_id int NOT NULL
);