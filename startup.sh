export ROCKET_PROFILE=production
nohup cargo run --features "env-file" > ../logs/run.log &
