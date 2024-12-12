# Rust-Backed Personal Website
This is super simple, just made to get up and running quickly. 

## Set Up Locally
Install rust first.
* `git clone github.com:ASchneidman/personal-website.git`
* `cd personal-website`
* `cargo run`

Currently, it listens on `7878`.

## Set Up on an AWS EC2 Instance
* Create a new EC2 instance, ideally with Ubuntu.
* Download the private `.pem` file associated with the instance. Save it somewhere good and don't share it :). I put it in a `secrets` folder in this project, of course added to `.gitignore`. 
* ssh into the instance
```
chmod 400 "<path to .pem>"
ssh -i "<path to .pem>" ubuntu@<EC2 Public DNS Address>
```
* Install rust
```
sudo apt-get install cargo
```
* Generate a ssh key on the EC2 instance and associate it with your github account. See https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account
* Follow the instructions in [Set Up Locally](#set-up-locally)