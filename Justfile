# Install `diesel_cli` if not present.
install:
	which diesel 2>&1 > /dev/null || cargo install diesel_cli --features postgres

# Initialize the database.
setup: install
	diesel setup

# Generate migration script. Migration name must follow the command.
migrate-gen: setup
	disesel mirgation Generate

# Actually execute migration.
migrate-do:
	diesel migration ru

# Revert the migration.
migrate-revert:
	diesel migration redo

help:
	@just -l

# vim: set ft=make:
