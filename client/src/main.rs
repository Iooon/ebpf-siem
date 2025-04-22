use awc::{http::StatusCode, Client};
use futures::future::join_all;
use models::Command;
use procfs::process::Process;
use std::{thread::sleep, time::Duration};

fn debugger() -> bool {
    true
}

async fn send_command(client: &Client, id: u32, str: String) -> Result<StatusCode, String> {
    if debugger() {
        println!("Process info: {:?} {:?}", id, str);
        Ok(StatusCode::CONTINUE)
    } else {
        let command = Command {
            userid: id,
            command: str.into(),
        };
        let result = client
            .post("http://127.0.0.1:5001/command") // <- Create request builder
            .send_form(&command)
            .await;
        match result {
            Ok(response) => Ok(response.status()),
            Err(_) => panic!("Could not send command"),
        }
    }
}

fn get_processes() -> Vec<Process> {
    procfs::process::all_processes()
        .expect("Can't read /proc")
        .filter_map(|p| match p {
            Ok(p) => Some(p), // happy path
            Err(e) => match e {
                procfs::ProcError::NotFound(_) => None, // process vanished during iteration, ignore it
                procfs::ProcError::Io(_, _) => None, // can match on path to decide if we can continue
                x => {
                    println!("Can't read process due to error {x:?}"); // some unknown error
                    None
                }
            },
        })
        .collect()
}

fn filter_new<T>(old: &Vec<T>, new: Vec<T>, filter: fn(&T, &T) -> bool) -> Vec<T> {
    let mut diff: Vec<T> = Vec::new();
    for n in new {
        if !old.iter().any(|o| filter(o, &n)) {
            diff.push(n);
        }
    }
    diff
}

#[actix_rt::main]
async fn main() {
    let client = Client::default();

    // The current processes
    let mut processes = get_processes();

    // Polling the new processes in the /proc location every second
    // This could easily miss short processes and should probabily be changed
    loop {
        sleep(Duration::from_secs(1));
        let new_processes = get_processes();
        let diff_processes = filter_new(&mut processes, get_processes(), |p1, p2| {
            p1.pid() == p2.pid()
        });
        // Send the new found processes and wait until all are send
        join_all(diff_processes.iter().map(|p| async {
            send_command(&client, p.pid() as u32, p.cmdline().unwrap().join(" "))
                .await
                .unwrap();
        }))
        .await;
        // Update the current processes
        processes.clear();
        processes = new_processes;
    }
}

#[cfg(test)]
mod tests {
    use awc::Client;

    #[actix_rt::test]
    async fn server_connect() {
        let client = Client::default();

        let client_response = client
            .get("http://127.0.0.1:5001") // <- Create request builder
            .insert_header(("User-Agent", "Actix-web"))
            .send() // <- Send http request
            .await;
        let mut res = client_response.unwrap();
        res.body().await.unwrap();
        //assert_eq!(response, b"Command added");
    }
}
