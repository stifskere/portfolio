
CREATE TABLE IF NOT EXISTS reviews (
	id SERIAL PRIMARY KEY,
	link_uuid VARCHAR(50) NOT NULL,
	can_edit BOOLEAN NOT NULL DEFAULT TRUE,
	submited_name VARCHAR(50),
	submited_rating INT,
	submited_comment VARCHAR(375),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
	CHECK (
		submited_rating >= 0
		AND submited_rating <= 5
	),
	CHECK (
		link_uuid IS NOT NULL
		OR (
			submited_name IS NOT NULL
			AND submited_rating IS NOT NULL
			AND submited_comment IS NOT NULL
		)
	)
);
