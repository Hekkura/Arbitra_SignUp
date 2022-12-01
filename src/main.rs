use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container-fluid h-100 pb-5"> 
                <div class="row">
                    <div style="background-color:#A73034" class="col-md text-light">
                        // ===========================================
                        <div class="row p-3 d-flex justify-content-center"> 
                            <img src="assets/Arbitra Full2 Wht.png" class="card-img-top" style="width: 200px" alt="..."/>

                        </div>
                        // ===========================================
                        <div class="row p-3 d-flex justify-content-md-start"> 
                            <h2><strong>{ "Get Started For Free."}</strong></h2>
                            <h4>{ "Search API for your every need." }</h4>
                        </div>
                        // ===========================================
                        <div class="row p-3 d-flex justify-content-between flex-row">
                            <div> 
                                <img src="assets/productivity.png" class="card-img-top" style="width: 60px" alt="..."/>
                            
                            </div>
                            
                            <div> 
                                <h4>{ "Manage your data." }</h4>
                                <p class="card-text"> { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, 
                                sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, 
                                quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. " } </p>
                                
                            </div> 
                        </div>
                        // ===========================================
                        <div class="row p-3 d-flex justify-content-between flex-row">
                            <div>
                                <img src="assets/predictive-chart.png" class="card-img-top" style="width: 60px" alt="..."/>
                            
                            </div>

                            <div> 
                                <h4>{ "Analytics & Services" }</h4>
                                <p class="card-text"> { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, 
                                sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, 
                                quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. " } </p>
                            
                            </div>
                            
                        </div>
                        // ===========================================
                    </div>
                    <div class="col-md-9">
                        <p class="card-text"> { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, 
                        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, 
                        quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. " } </p> 
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}