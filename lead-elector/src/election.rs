use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// Message types for communication between servers
#[derive(Clone, Debug)]
enum Message {
    RequestVote(u32), // Request for vote
    Vote(u32),        // Vote response
    Leader(u32),      // Leader announcement
}

// Shared state among servers
#[derive(Debug)]
struct SharedState {
    current_term: u32,    // Current term
    voted_for: Option<u32>, // Voted for candidate in current term
    leader_id: Option<u32>, // Leader ID
    votes_received: HashSet<u32>, // Set of votes received in the current term
}

impl SharedState {

    fn new(id: u32, peers: Vec<TcpStream>) -> Self {
        let state = Arc::new(Mutex::new(SharedState {
            current_term: 0,
            voted_for: None,
            leader_id: None,
            accepted: HashSet::new(),
        }));

        Server { id, peers, state }
    }

    fn start(&mut self) {
        // Listen for incoming connections
        let listener = TcpListener::bind(format!("127.0.0.1:{}", 9000 + self.id)).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0; 1024];
                    let n = stream.read(&mut buffer).unwrap();
                    let message: Message = bincode::deserialize(&buffer[..n]).unwrap();
                    self.handle_message(message, &mut stream);
                }
                Err(e) => {
                    println!("Failed to accept incoming connection: {:?}", e);
                }
            }
        }
    }

    fn handle_message(&mut self, message: Message, stream: &mut TcpStream) {
        let mut state = self.state.lock().unwrap();
        match message {
            Message::Prepare(term, candidate_id) => {
                if term > state.current_term {
                    state.current_term = term;
                    state.voted_for = None;
                    state.leader_id = None;
                    state.accepted.clear();
                }

                if state.voted_for.is_none() {
                    state.voted_for = Some(candidate_id);
                    let response = Message::Promise(state.current_term, None);
                    stream.write(&bincode::serialize(&response).unwrap()).unwrap();
                } else {
                    let response = Message::Promise(state.current_term, state.voted_for);
                    stream.write(&bincode::serialize(&response).unwrap()).unwrap();
                }
            }
            Message::Promise(term, previous_vote) => {
                if term == state.current_term {
                    if previous_vote.is_none() || previous_vote == state.voted_for {
                        state.accepted.insert(stream.peer_addr().unwrap().port());
                        if state.accepted.len() > self.peers.len() / 2



   // Handle incoming messages
   fn handle_message(&mut self, message: Message, stream: &mut TcpStream) {
    let mut state = self.state.lock().unwrap();
    match message {
        Message::Prepare(term, candidate_id) => {
            if term > state.current_term {
                state.current_term = term;
                state.voted_for = None;
                state.leader_id = None;
                state.accepted.clear();
            }

            if state.voted_for.is_none() {
                state.voted_for = Some(candidate_id);
                let response = Message::Promise(state.current_term, None);
                stream.write(&bincode::serialize(&response).unwrap()).unwrap();
            } else {
                let response = Message::Promise(state.current_term, state.voted_for);
                stream.write(&bincode::serialize(&response).unwrap()).unwrap();
            }
        }
        Message::Promise(term, previous_vote) => {
            if term == state.current_term {
                if previous_vote.is_none() || previous_vote == state.voted_for {
                    state.accepted.insert(stream.peer_addr().unwrap().port());
                    if state.accepted.len() > self.peers.len() / 2;

                }
            }
        }
    
}