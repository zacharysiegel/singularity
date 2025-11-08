#!/bin/zsh

set -eo pipefail

master_secret="${1?"Argument 1 required: master_secret"}" # todo: pull master key from .env file if exists
environments=("local" "stage" "production")
repo_dir=$(git rev-parse --show-toplevel)

cd "${repo_dir}"

if ! which cargo 1> /dev/null 2>&1; then
	echo 'The `cargo` program is required'
	echo "Install rustc/cargo/rustup: \`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh\`"
	exit 1
fi

function required_program_simple {
	local program_name="$1"
	if ! which "$program_name" 1> /dev/null 2>&1; then
		echo "The \`${program_name}\` program is required"
		exit 1
	fi
}
required_program_simple "podman"
required_program_simple "dbmate"

echo "Initializing submodules"
git submodule update --init --recursive

function sqlx_setup {
	echo 'Installing the SQLx CLI onto the system (used for caching database state for query validations)'
	echo 'You can run this command to check the status of ./.sqlx: `cargo sqlx prepare --workspace --check -- --all-targets --all-features`'
	echo '...or to regenerate ./.sqlx: `cargo sqlx prepare --workspace -- --all-targets --all-features`'
	cargo install sqlx-cli
}
sqlx_setup

function generate_env_from_template {
	echo "Generating .env"
	local master_secret_key="master_secret"
	local postgres__user_singularity_password_local_key="postgres__user\.singularity\.password\.local"
	local postgres__user_singularity_password_local=$(
		cargo run -p crypt -- decrypt --utf8 --key "$master_secret" "$postgres__user_singularity_password_local_key"
	)

	sed > ./.env \
		-E \
		-e "s/${master_secret_key}/${master_secret}/g" \
		-e "s/${postgres__user_singularity_password_local_key}/${postgres__user_singularity_password_local}/g" \
		./template.env
}
generate_env_from_template

function generate_compose_from_template {
	echo "Generating compose.yaml"
	cp ./compose.template.yaml ./compose.yaml

	for environment in "${environments[@]}"; do
		local postgres__user_singularity_password_env_key="postgres__user.singularity.password.${environment}"
		local postgres__user_singularity_password_env=$(
			cargo run -p crypt -- decrypt --utf8 --key "$master_secret" "$postgres__user_singularity_password_env_key"
		)
		echo "s/${postgres__user_singularity_password_env_key}/${postgres__user_singularity_password_env}/g"
		sed -E -I "" \
			-e "s/${postgres__user_singularity_password_env_key}/${postgres__user_singularity_password_env}/g" \
			./compose.yaml
	done
}
generate_compose_from_template

# All setup scripts should be idempotent and callable from the repo root directory
# 	Include any subdirectory setup scripts below:
