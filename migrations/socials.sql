
CREATE TABLE IF NOT EXISTS socials (
	id SERIAL NOT NULL,
	name VARCHAR(15) NOT NULL UNIQUE,
	description VARCHAR(50),
	-- Serialized to a yew_icons::IconId
	icon_id VARCHAR(30) NOT NULL,
	target VARCHAR(200) NOT NULL,
	ui_order INTEGER NOT NULL UNIQUE
);

-- THE TRIGGERS DO NOT APPLY WITH ATLASGO, WILL BE LEFT HERE
-- FOR MAYBE A FUTURE IMPLEMENTATION

-- Set the order on insert
CREATE OR REPLACE FUNCTION set_order_on_insert()
RETURNS TRIGGER AS $$
BEGIN
	IF NEW.ui_order IS NULL THEN
		NEW.ui_order := COALESCE((SELECT MAX(ui_order) FROM socials), 0) + 1;
	END IF;

	RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_set_order_on_insert ON socials;

CREATE TRIGGER trigger_set_order_on_insert
BEFORE INSERT ON socials
FOR EACH ROW
	EXECUTE FUNCTION set_order_on_insert();

-- On delete decrement higher order rows.
CREATE OR REPLACE FUNCTION adjust_order_on_delete()
RETURNS TRIGGER AS $$
BEGIN
	UPDATE socials
	SET ui_order = ui_order - 1
	WHERE ui_order > OLD.ui_order;
	RETURN OLD;
END
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_adjust_order_on_delete ON socials;

CREATE TRIGGER trigger_adjust_order_on_delete
AFTER DELETE ON socials
FOR EACH ROW
	EXECUTE FUNCTION adjust_order_on_delete();


-- On update the order swap the current inhabitant.
CREATE OR REPLACE FUNCTION reorder_on_update()
RETURNS TRIGGER AS $$
DECLARE
	row_count INTEGER;
	conflict_id INTEGER;
BEGIN
	IF NEW.ui_order = OLD.ui_order THEN
		RETURN NEW;
	END IF;

	SELECT COUNT(*) INTO row_count FROM socials;
	IF NEW.ui_order < 1 OR NEW.ui_order > row_count THEN
		RAISE EXCEPTION 'Order % out of range (1..%)', NEW.ui_order, row_count;
	END IF;

	SELECT id INTO conflict_id
	FROM socials
	WHERE
		ui_order = NEW.ui_order
		AND id <> OLD.id;

	IF conflict_id IS NOT NULL THEN
		UPDATE socials
		SET ui_order = OLD.ui_order
		WHERE id = conflict_id;
	END IF;

	RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_reorder_on_update ON socials;

CREATE TRIGGER trigger_reorder_on_update
BEFORE UPDATE OF ui_order ON socials
FOR EACH ROW
	EXECUTE FUNCTION reorder_on_update();

