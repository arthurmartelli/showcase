CREATE TABLE users (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255),
    email VARCHAR(255) UNIQUE,
    email_verified_at TIMESTAMP NULL,
    PASSWORD VARCHAR(255),
    remember_token VARCHAR(100),
    created_at TIMESTAMP NULL,
    updated_at TIMESTAMP NULL
);

CREATE TABLE password_resets (
    email VARCHAR(255) PRIMARY KEY,
    token VARCHAR(255),
    created_at TIMESTAMP NULL
);

CREATE TABLE failed_jobs (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    uuid VARCHAR(255) UNIQUE,
    connection TEXT,
    queue TEXT,
    payload LONGTEXT,
    exception LONGTEXT,
    failed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE products (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    product_code VARCHAR(255) UNIQUE,
    name VARCHAR(255),
    category VARCHAR(255),
    stock INT,
    unit_price VARCHAR(255),
    sales_unit_price VARCHAR(255),
    created_at TIMESTAMP NULL,
    updated_at TIMESTAMP NULL
);

CREATE TABLE customers (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255),
    email VARCHAR(255) UNIQUE,
    company VARCHAR(255),
    address VARCHAR(255),
    phone VARCHAR(255),
    created_at TIMESTAMP NULL,
    updated_at TIMESTAMP NULL
);

CREATE TABLE orders (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    email VARCHAR(255),
    product_code VARCHAR(255),
    product_name VARCHAR(255),
    quantity INT,
    order_status INT,
    created_at TIMESTAMP NULL,
    updated_at TIMESTAMP NULL
);

CREATE TABLE invoices (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    customer_name VARCHAR(255),
    customer_mail VARCHAR(255),
    company VARCHAR(255),
    address VARCHAR(255),
    item VARCHAR(255),
    product_name VARCHAR(255),
    price VARCHAR(255),
    quantity INT,
    total VARCHAR(255),
    payment VARCHAR(255),
    due VARCHAR(255),
    created_at TIMESTAMP NULL,
    updated_at TIMESTAMP NULL
);