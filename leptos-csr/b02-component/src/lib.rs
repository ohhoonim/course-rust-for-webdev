use leptos::prelude::*;
mod components;

use components::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"Components"</h1>
            <Card title="버튼" subtitle="All four variants".to_string()>
                <Button label="Primary" />
                <Button label="Secondary" variant=ButtonVariant::Secondary/>
                <Button label="Danger" variant=ButtonVariant::Danger/>
                <Button label="Ghost" variant=ButtonVariant::Ghost/>
                <Button label="Disabled" disabled=true/>
            </Card>
            <Card title="뱃지" subtitle="Color-coded status labels".to_string()>
                <Badge text="In progress" color=BadgeColor::Blue/>
                <Badge text="Shipped" color=BadgeColor::Green/>
                <Badge text="Warning" color=BadgeColor::Yellow/>
                <Badge text="Failed" color=BadgeColor::Red/>
                <Badge text="Draft" color=BadgeColor::Gray/>
            </Card>
            <Card title="경고 팝업" subtitle="Dismissible and non-dismissible".to_string()>
                <Alert message="Informational message." alert_type=AlertType::Info/>
                <Alert message="Operation succeeded!" alert_type=AlertType::Success dismissible=true/>
                <Alert message="This is a warning." alert_type=AlertType::Warning dismissible=true/>
                <Alert message="An error occurred." alert_type=AlertType::Error/>
            </Card>
            <Card title="아바타" subtitle="Initials fallback, three sizes".to_string()>
                <Avatar name="Darshan Vichhi" size=AvatarSize::Sm/>
                <Avatar name="Aarambh Dev Hub" size=AvatarSize::Md/>
                <Avatar name="Leptos Rust" size=AvatarSize::Lg/>
            </Card>
        </main>
    }
}
