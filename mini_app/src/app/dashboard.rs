use crate::components::navbar::Navbar;
use std::collections::HashSet;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Clone)]
pub struct ScheduledEvent {
    id: i32,
    date: String,
    name: String,
    description: String,
}

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let show_create_model = use_state(|| false);
    let show_update_model = use_state(|| false);
    let show_delete_model = use_state(|| false);
    let show_delete_selected_model = use_state(|| false);

    let toggle = |state: UseStateHandle<bool>| {
        let state = state.clone();
        Callback::from(move |_| state.set(!*state))
    };
    let toggle_create_model = toggle(show_create_model.clone());
    let toggle_update_model = toggle(show_update_model.clone());
    let toggle_delete_model = toggle(show_delete_model.clone());
    let toggle_delete_selected_model = toggle(show_delete_selected_model.clone());
    let update_model_form = use_node_ref();
    let create_model_form = use_node_ref();

    let models: UseStatePtrEqHandle<Vec<ScheduledEvent>> = use_state_ptr_eq(|| {
        vec![
            ScheduledEvent {
                id: 1,
                date: "2024-09-10".to_string(),
                name: "Conference".to_string(),
                description: "Annual tech conference".to_string(),
            },
            ScheduledEvent {
                id: 2,
                date: "2024-09-15".to_string(),
                name: "Workshop".to_string(),
                description: "Rust programming workshop".to_string(),
            },
            ScheduledEvent {
                id: 3,
                date: "2024-09-20".to_string(),
                name: "Hackathon".to_string(),
                description: "24-hour coding challenge".to_string(),
            },
            ScheduledEvent {
                id: 4,
                date: "2024-09-25".to_string(),
                name: "Webinar".to_string(),
                description: "Web development trends".to_string(),
            },
            ScheduledEvent {
                id: 5,
                date: "2024-09-30".to_string(),
                name: "Meetup".to_string(),
                description: "Local developer meetup".to_string(),
            },
        ]
    });
    let delete_model_id = use_state_ptr_eq(|| None::<ScheduledEvent>);
    let current_model = use_state_ptr_eq(|| ScheduledEvent {
        id: 0,
        name: String::new(),
        date: String::new(),
        description: String::new(),
    });

    let selected_models = use_state(HashSet::new);
    let select_all = use_state(|| false);
    let on_select_model = {
        let selected_models = selected_models.clone();
        Callback::from(move |model: ScheduledEvent| {
            let mut new_selected_id = (*selected_models).clone();
            if new_selected_id.contains(&model.id) {
                new_selected_id.remove(&model.id);
            } else {
                new_selected_id.insert(model.id);
            }
            selected_models.set(new_selected_id);
        })
    };

    let on_select_all = {
        let models = models.clone();
        let selected_models = selected_models.clone();
        let select_all = select_all.clone();
        Callback::from(move |_: MouseEvent| {
            let new_select_all = !*select_all;
            select_all.set(new_select_all);
            let mut selected_ids = HashSet::new();
            if new_select_all {
                for model in &*models {
                    selected_ids.insert(model.id.try_into().unwrap());
                }
            }
            selected_models.set(selected_ids);
        })
    };
    let modal_class_create = if *show_create_model { "" } else { "hidden" };
    let modal_class_update = if *show_update_model { "" } else { "hidden" };
    let modal_class_delete = if *show_delete_model { "" } else { "hidden" };
    let on_update_model = {
        let current_model = current_model.clone();
        let show_update_model = show_update_model.clone();
        Callback::from(move |model| {
            current_model.set(model);
            show_update_model.set(!*show_update_model);
        })
    };
    let on_delete_model = {
        let delete_model_id = delete_model_id.clone();
        let show_delete_model = show_delete_model.clone();
        Callback::from(move |model: ScheduledEvent| {
            delete_model_id.set(Some(model));
            show_delete_model.set(!*show_delete_model);
        })
    };
    html! {

                    <div>
                        <Navbar />
                                                          <div class=" mt-14 p-4 bg-white block sm:flex items-center justify-between border-b border-gray-200 lg:mt-12 dark:bg-gray-800 dark:border-gray-700">
                                                      <div class="w-full mb-1">
                                                          <div class="mb-4">
                                                              <h1 class="text-xl font-semibold text-gray-900 sm:text-2xl dark:text-white">{"Компанії"}</h1>

                                                         </div>
                                                          <div class="sm:flex dark:bg-gray-800">

                                                              <div class=" flex items-center ml-auto space-x-2 sm:space-x-3">
                                                                  <button type="button" onclick={toggle_create_model.clone()} data-modal-target="add-user-modal" data-modal-toggle="add-user-modal" class="inline-flex items-center justify-center w-1/2 px-3 py-2 text-sm font-medium text-center text-white rounded-lg bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 sm:w-auto dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
                                                                      <svg class="w-5 h-5 mr-2 -ml-1" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M10 5a1 1 0 011 1v3h3a1 1 0 110 2h-3v3a1 1 0 11-2 0v-3H6a1 1 0 110-2h3V6a1 1 0 011-1z" clip-rule="evenodd"></path></svg>
                                                          {"Додати"}
                                                                  </button>
                                                              </div>
                                                          </div>
                                                      </div>
                                                  </div>


                             // MODEL TABLE
                             // ==========================================================================================================================================
                                     <div class="flex flex-col">
                                             <div class="overflow-x-auto">
                                                 <div class="inline-block min-w-full align-middle">
                                                     <div class="overflow-hidden shadow">
                                                         <table class="min-w-full divide-y divide-gray-200 table-fixed dark:divide-gray-600">
                                                             <thead class="bg-gray-100 dark:bg-gray-700">
                                                                 <tr>
                                                                     <th scope="col" class="p-4">
                                                                         <div class="flex items-center">

                                                                             <label  class="text-gray-900 dark:text-gray-300">{"#"}</label>

                                                                         </div>
                                                                     </th>

                                                                     <th scope="col" class="p-4 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                                                         {"Назва"}
                                                                     </th>
                                                                     <th scope="col" class="p-4 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                                                         {"Дата"}
                                                                     </th>
                                                                     <th scope="col" class="p-4 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                                                         {"Дії"}
                                                                     </th>
                                                                 </tr>
                                                             </thead>
                                                             <tbody class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
                                                                 {

                                                                 for models.iter().enumerate().map(|(number,model)| html!{

                                                                 <tr class="hover:bg-gray-100 dark:hover:bg-gray-700">

                                                                     <td class="w-4 p-4">
                                                                             <label  class="text-gray-900 dark:text-gray-300">{number+1}</label>
                                                                     </td>

                                                                 <td class="p-4 text-base font-medium text-gray-900 whitespace-nowrap dark:text-white">
                                                                         { &model.name }
                                                                 </td>
                                                                     <td class="p-4 text-base font-medium text-gray-900 whitespace-nowrap dark:text-white">{ &model.date }</td>
                                                                     <td class="p-4 space-x-2 whitespace-nowrap">
                                                                         <button onclick={ let on_update_model = on_update_model.clone();
                                                                                           let model = model.clone();
                                                                                           Callback::from( move |_|{
                                                                                             on_update_model.emit(model.clone())})
                                                                                         }
                                                                 type="button" data-modal-target="edit-user-modal"  data-modal-toggle="edit-user-modal" class="inline-flex items-center px-2 py-2 text-sm font-medium text-center text-white rounded-lg bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">

                                                                             <svg class="w-4 h-4 " fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path d="M17.414 2.586a2 2 0 00-2.828 0L7 10.172V13h2.828l7.586-7.586a2 2 0 000-2.828z"></path><path fill-rule="evenodd" d="M2 6a2 2 0 012-2h4a1 1 0 010 2H4v10h10v-4a1 1 0 112 0v4a2 2 0 01-2 2H4a2 2 0 01-2-2V6z" clip-rule="evenodd"></path></svg>

                                                                         </button>
                                                                         <button onclick={ let on_delete_model = on_delete_model.clone();
                                                                                            let model = model.clone();
                                                                                            Callback::from( move |_| { let model = model.clone();
                                                                                            on_delete_model.emit(model)}) }

                                                                     type="button" data-modal-target="delete-user-modal" data-modal-toggle="delete-user-modal" class="inline-flex items-center px-2 py-2 text-sm font-medium text-center text-white bg-red-600 rounded-lg hover:bg-red-800 focus:ring-4 focus:ring-red-300 dark:focus:ring-red-900">
                                                                             <svg class="w-4 h-4 " fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd"></path></svg>

                                                                         </button>
                                                                     </td>
                                                                 </tr>
                                                              })}
                                                             </tbody>
                                                         </table>
                                                     </div>
                                                 </div>
                                             </div>
                                         </div>


        <div class={format!("{} fixed flex bg-black bg-opacity-30 shadow left-0 right-0 z-50 items-center justify-center overflow-x-hidden overflow-y-auto inset-0 h-full", modal_class_create) } id="add-user-modal">
                                         <div class="relative w-full  max-w-2xl px-4 h-auto">
                                             <div class="relative bg-white rounded-lg shadow dark:bg-gray-800">
                                                 <div class="flex items-start justify-between p-5 border-b rounded-t dark:border-gray-700">
                                                     <h3 class="text-xl font-semibold dark:text-white">
                                                         {"Додати компанію"}
                                                     </h3>
                                                     <button type="button" onclick={toggle_create_model} class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm p-1.5 ml-auto inline-flex items-center dark:hover:bg-gray-700 dark:hover:text-white" data-modal-toggle="add-user-modal">
                                                         <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
                                                     </button>
                                                 </div>
                                                 <form ref={create_model_form} class="group" //onsubmit={onclick_create}
                 novalidate=true>
                                                 <div class="p-6 space-y-6">
                                                                 <div class="grid grid-cols-6 gap-6">

                        <input id="update_id" name="update_id" class="hidden" value={format!("{}", current_model.id)} />
                                                                 <div class="col-span-6 sm:col-span-6">
                                                                     <label for="first-name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Назва події"}</label>
                                                                     <input     type="text" name="create_name" id="create_name" class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Назва компанії" required=true />
                                            <label for="date" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Дата"}</label>
                                                                     <input     type="text" name="create_date" id="create_date" class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Назва компанії" required=true />
            <label  for="message" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Опис"}</label>
    <textarea  id="message" rows="4" class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Write your thoughts here..."></textarea>
                                         </div>

                                                                 </div>
                                                             <div class="items-center p-6 border-t border-gray-200 rounded-b dark:border-gray-700">
                                                                 <button  class="cursor-pointer text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" type="submit">{"Зберегти"}</button>
                                                             </div>

                                                     </div>
                                             </form>
                                         </div>
                                     </div>
                                 </div>

            <div class={format!("{} fixed flex left-0 right-0 z-50 items-center  bg-black bg-opacity-30 shadow justify-center overflow-x-hidden overflow-y-auto inset-0  h-full", modal_class_update)} id="edit-user-modal">
                                                 <div class="relative w-full max-w-2xl px-4 h-auto">
                                                     <div class="relative bg-white rounded-lg shadow dark:bg-gray-800">
                             <div class="flex items-start justify-between p-5 border-b rounded-t dark:border-gray-700">
                                                             <h3 class="text-xl font-semibold dark:text-white">
                                                                 {"Редагувати компанію"}
                                                             </h3>
                                                             <button onclick={toggle_update_model} type="button" class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm p-1.5 ml-auto inline-flex items-center dark:hover:bg-gray-700 dark:hover:text-white" data-modal-toggle="edit-user-modal">
                                                                 <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
                                                             </button>
                                                         </div>
                                                         <form ref={update_model_form}  //onsubmit={on_confirm_update}
                    novalidate=true>
                                                         <div class="p-6 space-y-6">
                                                                 <div class="grid grid-cols-6 gap-6">

                        <input id="update_id" name="update_id" class="hidden" value={format!("{}", current_model.id)} />
                                                                 <div class="col-span-6 sm:col-span-6">
                                                                     <label for="first-name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Назва події"}</label>
                                                                     <input  value={ current_model.name.clone()}   type="text" name="name" id="name" class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Назва компанії" required=true />
                                            <label for="first-name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Дата"}</label>
                                                                     <input  value={ current_model.date.clone()}   type="text" name="update_model_name" id="update_model_name" class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Назва компанії" required=true />
            <label  for="message" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Опис"}</label>
    <textarea value={current_model.description.clone()} id="message" rows="4" class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Write your thoughts here..."></textarea>
                                         </div>

                                                                 </div>
                                                             <div class="items-center p-6 border-t border-gray-200 rounded-b dark:border-gray-700">
                                                                 <button  class="cursor-pointer text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" type="submit">{"Зберегти"}</button>
                                                             </div>

                                                     </div>
                                                 </form>
                                                 </div>
                                             </div>
                                         </div>

        <div class={format!("{} fixed flex bg-black bg-opacity-30 shadow left-0 right-0 z-50 items-center justify-center  overflow-x-hidden overflow-y-auto inset-0 h-full", modal_class_delete)} id="delete-user-modal">
                        <div class="relative w-full max-w-md px-4 h-auto">
                            <div class="relative bg-white rounded-lg shadow dark:bg-gray-800">
                                <div class="flex justify-end p-2">
                                    <button onclick={&toggle_delete_model} type="button" class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm p-1.5 ml-auto inline-flex items-center dark:hover:bg-gray-700 dark:hover:text-white" data-modal-hide="delete-user-modal">
                                        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
                                    </button>
                                </div>
                                <div class="p-6 pt-0 text-center">
                                    <svg class="w-16 h-16 mx-auto text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                    <h3 class="mt-5 mb-6 text-lg text-gray-500 dark:text-gray-400">{"Підвержуєте видалення?"}</h3>
                                    <a //onclick={&props.delete}
            class="cursor-pointer text-white bg-red-600 hover:bg-red-800 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-base inline-flex items-center px-3 py-2.5 text-center mr-2 dark:focus:ring-red-800">
                                        {"Так"}
                                    </a>
                                    <a onclick={&toggle_delete_model} class="cursor-pointer text-gray-900 bg-white hover:bg-gray-100 focus:ring-4 focus:ring-blue-300 border border-gray-200 font-medium inline-flex items-center rounded-lg text-base px-3 py-2.5 text-center dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700 dark:focus:ring-gray-700" data-modal-hide="delete-user-modal">
                                    {"Ні"}
                                    </a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                }
}
