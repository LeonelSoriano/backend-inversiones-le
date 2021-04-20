-- Your SQL goes here
CREATE TABLE unit (
    id BIGINT PRIMARY KEY NOT NULL,
    name VARCHAR(120) NOT NULL,
    sign VARCHAR(24) NOT NULL
);

INSERT INTO public.unit (id, "name", sign) VALUES(1, 'centimetro', 'cm');
INSERT INTO public.unit (id, "name", sign) VALUES(2, 'caja', 'caja');
INSERT INTO public.unit (id, "name", sign) VALUES(3, 'unidad', 'und');
INSERT INTO public.unit (id, "name", sign) VALUES(4, 'metro', 'm');
INSERT INTO public.unit (id, "name", sign) VALUES(5, 'paquete', 'paquete');

CREATE TABLE business (
    id BIGINT PRIMARY KEY NOT NULL,
    name VARCHAR(120) NOT NULL,
    sign VARCHAR(24) NOT NULL
);

INSERT INTO public.business (id, "name", sign) VALUES(1, 'Market place', 'MP');
INSERT INTO public.business (id, "name", sign) VALUES(2, 'Mercado libre', 'ML');

CREATE TABLE products (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(120) NOT NULL,
    description VARCHAR(255) NOT NULL,
    purchase_unit BIGINT NOT NULL  REFERENCES unit(id),
    retail_unit BIGINT NOT NULL  REFERENCES unit(id),
    sale_unit BIGINT NOT NULL  REFERENCES unit(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);


CREATE TABLE products_bussiness_table (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(155) NOT NULL,
    product_fk  BIGINT NOT NULL  REFERENCES products(id),
    business_fk BIGINT NOT NULL REFERENCES business(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);


CREATE TABLE img_products (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(155) NOT NULL,
    product_fk  BIGINT NOT NULL  REFERENCES products(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

/*
CREATE TABLE login_history
(
    id SERIAL PRIMARY KEY NOT NULL,
    user_id BIGINT NOT NULL REFERENCES users(id),
    login_timestamp TIMESTAMP WITH TIME ZONE NOT NULL
);
*/
