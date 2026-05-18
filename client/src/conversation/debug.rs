// todo: remove this module
use crate::conversation::state::{Conversation, ConversationEvent, ConversationEventKey};
use crate::state::STATE;
use chrono::{Duration, Utc};
use shared::schema::account::AccountPublicSerial;
use shared::schema::conversation::{ConversationMember, ConversationMemberChange};
use shared::schema::conversation_message::ConversationMessage;
use uuid::Uuid;

pub fn seed_debug_conversations() {
    let conversation_ids: [Uuid; 4] = [
        Uuid::now_v7(),
        Uuid::now_v7(),
        Uuid::now_v7(),
        Uuid::now_v7(),
    ];

    let me_id: Uuid = resolve_or_set_own_account_id();
    let alpha_id: Uuid = Uuid::now_v7();
    let beta_id: Uuid = Uuid::now_v7();
    let gamma_id: Uuid = Uuid::now_v7();
    seed_account(me_id, "me");
    seed_account(alpha_id, "alpha");
    seed_account(beta_id, "beta");
    seed_account(gamma_id, "gamma");

    let mut conversation_a: Conversation = Conversation::new();
    conversation_a.name = Some("Strategy Squad".to_string());
    conversation_a.created = Some(Utc::now());
    conversation_a.unread_count = 3;
    seed_strategy_squad_messages(&mut conversation_a, conversation_ids[0], me_id, alpha_id, beta_id, gamma_id);
    STATE.conversation.conversations.insert(conversation_ids[0], conversation_a);

    let mut conversation_b: Conversation = Conversation::new();
    conversation_b.name = Some("Game: Expansion".to_string());
    conversation_b.created = Some(Utc::now());
    conversation_b.unread_count = 12;
    seed_expansion_messages(&mut conversation_b, conversation_ids[1], me_id, alpha_id, beta_id);
    STATE.conversation.conversations.insert(conversation_ids[1], conversation_b);

    let mut conversation_c: Conversation = Conversation::new();
    conversation_c.name = Some("player_alpha".to_string());
    conversation_c.created = Some(Utc::now());
    seed_dm_messages(&mut conversation_c, conversation_ids[2], me_id, alpha_id);
    STATE.conversation.conversations.insert(conversation_ids[2], conversation_c);

    let mut conversation_d: Conversation = Conversation::new();
    conversation_d.name = Some("A very long conversation name that should get truncated by the ellipsis".to_string());
    conversation_d.created = Some(Utc::now());
    conversation_d.unread_count = 1;
    STATE.conversation.conversations.insert(conversation_ids[3], conversation_d);

    log::info!("Seeded {} debug conversations", conversation_ids.len());
}

fn seed_account(account_id: Uuid, username: &str) {
    STATE.account.cache.insert(
        account_id,
        AccountPublicSerial {
            id: account_id,
            username: username.to_string(),
        },
    );
}

/// Returns the current `own_account_id` if catch-up already set it, otherwise generates
/// one and stores it so the rest of the seed can attribute "me" messages consistently.
fn resolve_or_set_own_account_id() -> Uuid {
    let mut own_account_id_guard = STATE.account.own_account_id.write().unwrap();
    match *own_account_id_guard {
        Some(account_id) => account_id,
        None => {
            let account_id: Uuid = Uuid::now_v7();
            *own_account_id_guard = Some(account_id);
            account_id
        }
    }
}

fn seed_strategy_squad_messages(conversation: &mut Conversation, conversation_id: Uuid, me: Uuid, alpha: Uuid, beta: Uuid, gamma: Uuid) {
    let member_changes: &[(Uuid, MemberChangeKind, i64)] = &[
        (alpha, MemberChangeKind::Joined, 3600 * 8),
        (beta,  MemberChangeKind::Joined, 3600 * 8 - 600),
        (gamma, MemberChangeKind::Joined, 3600 * 7 + 1800),
        (me,    MemberChangeKind::Joined, 3600 * 7),
        (gamma, MemberChangeKind::Left,   9000),
        (gamma, MemberChangeKind::Joined, 4500),
    ];
    insert_member_changes(conversation, conversation_id, member_changes);

    let lines: &[(Uuid, &str, i64)] = &[
        (alpha, "anyone seeing the U-235 spike on the eastern hex cluster?", 3600 * 6),
        (alpha, "third night in a row.", 3600 * 6 - 5),
        (alpha, "i'm starting to think it's not random.", 3600 * 6 - 10),
        (beta,  "yeah, third tick in a row. someone's been pre-positioning extractors there.", 3600 * 6 - 30),
        (gamma, "could be a feint. last time we saw that pattern it ended up being a supply screen for a flank push.", 3600 * 6 - 90),
        (me,    "i was about to ask the same. let me pull telemetry.", 3600 * 6 - 110),
        (me,    "ok pulled. yeah, definite spike on hexes 13/14/15.", 3600 * 6 - 105),
        (me,    "and look at the timing of the convoy departures - it lines up.", 3600 * 6 - 100),
        (alpha, "we should sink a scout into hex 14 just to be sure. cheap insurance.", 3600 * 5 - 50),
        (beta,  "agreed. i can spare one - give me the tick budget and i'll route it.", 3600 * 5 - 110),
        (me,    "tick budget on hex 14 is 6 right now, but i can free up 2 more by deferring the refinery handoff.", 3600 * 5 - 140),
        (gamma, "do not commit yet - the trade window with the southern coalition is still open and i don't want to telegraph our worry.", 3600 * 4),
        (gamma, "give me 30 minutes.", 3600 * 4 - 5),
        (alpha, "fair. we wait until the trade closes, then move the scout the next tick.", 3600 * 4 - 60),
        (beta,  "noted. queued for tick +2.", 3600 * 4 - 120),
        (me,    "queue confirmed on my side too. i'll babysit the build order until the swap lands.", 3600 * 4 - 160),
        (gamma, "while we're at it, we should also start thinking about the post-trade U-235 distribution. if we hit our 240 threshold this week we can flip a build order to centrifuges instead of refiners.", 3600 * 3),
        (alpha, "i'll model both paths tonight and post numbers in the morning.", 3600 * 3 - 200),
        (me,    "if you want a sanity check on the centrifuge throughput numbers, i ran the math last week and have a spreadsheet i can drop in here.", 3600 * 3 - 240),
        (beta,  "+1", 3600 * 2 - 30),
        (beta,  "actually pls share the sheet, i want to see assumptions.", 3600 * 2 - 25),
        (alpha, "also: somebody please poke at the lobby UI, the conversation tab tooltip is being weird on hover after a tab dismiss.", 1800),
        (gamma, "i can repro. filing it.", 1700),
        (beta,  "not blocking - strategy first, polish later.", 1500),
        (me,    "agree, parking the UI thread until after the trade window closes.", 1450),
        (alpha, "this is a long one, just to make sure word wrapping handles a multi-line message gracefully even when somebody decides to dump several sentences in a row without any newline characters whatsoever and just keeps going and going and going.", 600),
        (gamma, "lol", 540),
        (beta,  "wrapped messages incoming, brace.", 480),
        (me,    "and here's a long one from me too, so we can confirm right-aligned wrapping looks correct when the body has to span multiple lines and the sender header line drifts off to the right edge cleanly.", 420),
        (me,    "follow-up.", 415),
        (me,    "and one more, to test that consecutive own-message bundling looks right with the right-edge alignment all the way down.", 410),
    ];
    insert_messages(conversation, conversation_id, lines);
}

fn seed_expansion_messages(conversation: &mut Conversation, conversation_id: Uuid, me: Uuid, alpha: Uuid, beta: Uuid) {
    let lines: &[(Uuid, &str, i64)] = &[
        (alpha, "moving the second extractor to the ridge tonight", 7200),
        (beta,  "watch for the patrol on tick 18 - they swing wide there.", 7100),
        (me,    "i'll cover the flank with two scouts.", 7080),
        (alpha, "noted, will route around.", 7050),
        (beta,  "build queue updated; trade caravan dispatched.", 3600),
        (me,    "caravan ETA tick 22, i'll meet it at the junction.", 3500),
        (alpha, "we're live.", 600),
    ];
    insert_messages(conversation, conversation_id, lines);
}

fn seed_dm_messages(conversation: &mut Conversation, conversation_id: Uuid, me: Uuid, alpha: Uuid) {
    let member_changes: &[(Uuid, MemberChangeKind, i64)] = &[
        (alpha, MemberChangeKind::Joined, 86_500),
        (me,    MemberChangeKind::Joined, 86_500),
    ];
    insert_member_changes(conversation, conversation_id, member_changes);

    let lines: &[(Uuid, &str, i64)] = &[
        (alpha, "gg earlier - that flank from the north was nasty.", 86_400),
        (me,    "yeah you almost had me, the second push caught me out of position.", 86_350),
        (alpha, "want to team up next round?", 86_300),
        (me,    "absolutely. queue in 10?", 86_280),
    ];
    insert_messages(conversation, conversation_id, lines);
}

fn insert_messages(conversation: &mut Conversation, conversation_id: Uuid, lines: &[(Uuid, &str, i64)]) {
    let now = Utc::now();
    for (sender_account_id, content, seconds_ago) in lines {
        let created = now - Duration::seconds(*seconds_ago);
        let message = ConversationMessage {
            id: Uuid::now_v7(),
            conversation_id,
            sender_account_id: *sender_account_id,
            content: content.to_string(),
            created,
        };
        let event = ConversationEvent::Chat(message);
        let key = ConversationEventKey::from(&event);
        conversation.events.insert(key, event);
    }
}

#[derive(Debug, Clone, Copy)]
enum MemberChangeKind {
    Joined,
    Left,
}

fn insert_member_changes(
    conversation: &mut Conversation,
    conversation_id: Uuid,
    changes: &[(Uuid, MemberChangeKind, i64)],
) {
    let now = Utc::now();
    for (account_id, kind, seconds_ago) in changes {
        let timestamp = now - Duration::seconds(*seconds_ago);
        let change = ConversationMemberChange {
            conversation_id,
            account_id: *account_id,
            timestamp,
        };
        let event = match kind {
            MemberChangeKind::Joined => {
                conversation.members.insert(*account_id, ConversationMember {
                    conversation_id,
                    account_id: *account_id,
                    entered: timestamp,
                    exited: None,
                    color_cached: None,
                });
                ConversationEvent::MemberJoined(change)
            }
            MemberChangeKind::Left => {
                conversation.members.remove(account_id);
                ConversationEvent::MemberLeft(change)
            }
        };
        let key = ConversationEventKey::from(&event);
        conversation.events.insert(key, event);
    }
}
