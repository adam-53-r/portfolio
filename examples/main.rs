use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos::html;
use leptos::tachys::html::event::SubmitEvent;

/// Main component
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    let values = vec![0, 1, 2];

    // create a list of 5 signals
    let length = 5;
    let counters = (1..=length).map(|idx| RwSignal::new(idx));
    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|count| {
            view! {
                <li>
                    <button
                        on:click=move |_| *count.write() += 1
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();


    let (name, set_name) = signal("Controlled".to_string());
    let email = RwSignal::new("".to_string());
    let spam_me = RwSignal::new(true);

    let (name, set_name) = signal("Uncontrolled".to_string());

    let input_element: NodeRef<html::Input> = NodeRef::new();
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();
    
        // here, we'll extract the value from the input
        let value = input_element
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name.set(value);
    };

    view! {
        <button
            on:click = move |_| *set_count.write() += 1
            // class:red = move || count.get() % 2 == 1
            // class=("red", move || count.get() % 2 == 1)
            style:color = move || if count.get() % 2 == 1 {"red"} else {""}
            // style=("color:blue", "true")
        >
            "Click me: "{count}
        </button>

        <ProgressBar progress=count/>
        <ProgressBar progress=Signal::derive(double_count)/>
        <ProgressBar2 progress=count.into() />
        <ProgressBar2 progress=Signal::derive(double_count)/>

        // this will just render "012"
        <p>{values.clone()}</p>
        // or we can wrap them in <li>
        <ul>
            {values.clone().into_iter()
                .map(|n| view! { <li>{n}</li>})
                .collect::<Vec<_>>()}
        </ul>
        // or we can wrap them in <li>
        <ul>
            {values.into_iter()
                .map(|n| view! { <li>{n}</li>})
                .collect_view()}
        </ul>

        <ul>{counter_buttons}</ul>

        <h1>"Iteration"</h1>
        <h2>"Static List"</h2>
        <p>"Use this pattern if the list itself is static."</p>
        <StaticList length=5/>
        <h2>"Dynamic List"</h2>
        <p>"Use this pattern if the rows in your list will change."</p>
        <DynamicList initial_length=5/>
        <h2>"Simple For List"</h2>
        <ForComponent/>

        <input type="text"
            bind:value=(name, set_name)
        />
        <input type="email"
            bind:value=email
        />
        <label>
            "Please send me lots of spam email."
            <input type="checkbox"
                bind:checked=spam_me
            />
        </label>
        <p>"Name is: " {name}</p>
        <p>"Email is: " {email}</p>
        <Show when=move || spam_me.get()>
            <p>"You’ll receive cool bonus content!"</p>
        </Show>

        <form on:submit=on_submit> // on_submit defined below
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>

    }
}

/// Progress bar with custom progress count and max attribute
#[component]
fn ProgressBar(
    /// Progress signal prop
    #[prop(into)]
    progress: Signal<i32>,
    /// Max attribute for progress element
    #[prop(default = 50)]
    max: u16,
) -> impl IntoView {
    view! {
        <progress
            max=max
            // we use it once here
            value=progress
        />
    }
}

/// Progress bar with optional custom progress count and max attribute
#[component]
fn ProgressBar2(
    /// Optional progress signal prop
    #[prop(optional)]
    progress: Signal<i32>,
) -> impl IntoView {
    // progress.map(|progress| {
    view! {
        <progress
            max=50
            value=progress
        />
        <br/>
    }
    // })
}

/// A list of counters, without the ability
/// to add or remove any.
#[component]
fn StaticList(
    /// How many counters to include in this list.
    length: usize,
) -> impl IntoView {
    // create counter signals that start at incrementing numbers
    let counters = (1..=length).map(|idx| RwSignal::new(idx));

    // when you have a list that doesn't change, you can
    // manipulate it using ordinary Rust iterators
    // and collect it into a Vec<_> to insert it into the DOM
    let counter_buttons = counters
        .map(|count| {
            view! {
                <li>
                    <button
                        on:click=move |_| *count.write() += 1
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect::<Vec<_>>();

    // Note that if `counter_buttons` were a reactive list
    // and its value changed, this would be very inefficient:
    // it would rerender every row every time the list changed.
    view! {
        <ul>{counter_buttons}</ul>
    }
}

/// A list of counters that allows you to add or
/// remove counters.
#[component]
fn DynamicList(
    /// The number of counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    // This dynamic list will use the <For/> component.
    // <For/> is a keyed list. This means that each row
    // has a defined key. If the key does not change, the row
    // will not be re-rendered. When the list changes, only
    // the minimum number of changes will be made to the DOM.

    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    // see NOTE in add_counter below re: ArcRwSignal
    let initial_counters = (0..initial_length)
        .map(|id| (id, ArcRwSignal::new(id + 1)))
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = signal(initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        // we use ArcRwSignal here, instead of RwSignal
        // ArcRwSignal is a reference-counted type, rather than the arena-allocated
        // signal types we've been using so far.
        // When we're creating a collection of signals like this, using ArcRwSignal
        // allows each signal to be deallocated when its row is removed.
        let sig = ArcRwSignal::new(next_counter_id + 1);
        // add this counter to the list of counters
        set_counters.update(move |counters| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            counters.push((next_counter_id, sig))
        });
        // increment the ID so it's always unique
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>
                "Add Counter "
            </button>
            <ul>
                // The <For/> component is central here
                // This allows for efficient, key list rendering
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each=move || counters.get()
                    // the key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|counter| counter.0
                    // `children` receives each item from your `each` iterator
                    // and returns a view
                    children=move |(id, count)| {
                        // we can convert our ArcRwSignal to a Copy-able RwSignal
                        // for nicer DX when moving it into the view
                        let count = RwSignal::from(count);
                        view! {
                            <li>
                                <button
                                    on:click=move |_| *count.write() += 1
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters
                                            .write()
                                            .retain(|(counter_id, _)| {
                                                counter_id != &id
                                            });
                                    }
                                >
                                    "Remove"
                                </button>
                                {id}
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Counter {
    id: usize,
    count: RwSignal<i32>,
}

#[component]
fn ForComponent() -> impl IntoView {
    let init_counters = (1..5)
        .map(|id| Counter {
            id,
            count: RwSignal::new(id as i32),
        })
        .collect();
    let (counters, set_counters) = signal::<Vec<Counter>>(init_counters);

    view! {
      <div>
        <button
          on:click= move |_| {
            let next_index = counters.get().last().map_or(1, |c| c.id+1);
            set_counters.write().push(
              Counter { id: next_index, count: RwSignal::new(next_index as i32) }
            );
          }
        >
          "Add button"
        </button>
        <p/>
        <For
          // a function that returns the items we're iterating over; a signal is fine
          each=move || counters.get()
          // a unique key for each item
          key=|counter| counter.id
          // renders each item to a view
          children=move |counter: Counter| {
            view! {
              <button
                on:click= move |_| {
                  *counter.count.write() += 1;
                }
              >
                "Value: " {move || counter.count.get()}
              </button>
              <button
                on:click= move |_| {
                  set_counters
                    .write()
                    .retain(|(vec_counter)| {
                      vec_counter.id != counter.id
                    });
                }
              >
                "Remove"
              </button>
              <p/>
            }
          }
        />
      </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
