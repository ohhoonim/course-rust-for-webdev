use leptos::prelude::*;
mod components;

use components::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
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
            <Card title="탭" subtitle="탭".to_string()>
                <Tabs>
                <Tab slot:tabs name="개요".to_string()>
                    <p>"첫 번째 탭의 내용입니다. 슬롯을 통해 전달되었습니다."</p>
                </Tab>

                <Tab slot:tabs name="카운터".to_string()>
                    <p>"현재 카운트: " <strong>{count}</strong></p>
                    <button type="button" on:click=move |_| set_count.update(|n| *n += 1)>
                        "+1 증가"
                    </button>
                </Tab>

                <Tab slot:tabs name="안내".to_string()>
                    <ul>
                        <li>"탭 버튼을 눌러 콘텐츠를 전환할 수 있습니다."</li>
                        <li>"빈 탭도 Option<ChildrenFn> 덕분에 안전하게 처리됩니다."</li>
                    </ul>
                </Tab>
            </Tabs>
            </Card>
        </main>
    }
}
