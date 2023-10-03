use yew::NodeRef;

#[derive(Default)]
pub struct HtmlSchema {
    pub main: MainSection,
    pub footer: FooterSection,
}

#[derive(Default)]
pub struct MainSection {
    pub h1: NodeRef,
    pub coord_div: NodeRef,
    pub canvas: CanvasElement,
    pub status_div: NodeRef,
    pub control_div: ControlSection,
}

#[derive(Default)]
pub struct CanvasElement {
    pub id: NodeRef,
    pub width: NodeRef,
    pub height: NodeRef,
    pub ref_attr: NodeRef,
}

#[derive(Default)]
pub struct ControlSection {
    pub label: NodeRef,
    pub select: NodeRef,
    pub threed_control_div: ThreedControlSection,
    pub mandelbrot_control_div: MandelbrotControlSection,
}

#[derive(Default)]
pub struct SelectElement {
    pub id: NodeRef,
    pub ref_attr: NodeRef,
    pub options: Vec<SelectOption>,
}

#[derive(Default)]
pub struct SelectOption {
    pub value: NodeRef,
    pub text: NodeRef,
}

#[derive(Default)]
pub struct ThreedControlSection {
    pub label_pitch: NodeRef,
    pub range_pitch: InputRangeElement,
    pub label_yaw: NodeRef,
    pub range_yaw: InputRangeElement,
    pub hidden: NodeRef,
    pub ref_attr: NodeRef,
}

#[derive(Default)]
pub struct InputRangeElement {
    pub type_attr: NodeRef,
    pub min: NodeRef,
    pub max: NodeRef,
    pub id: NodeRef,
    pub ref_attr: NodeRef,
    pub onchange: NodeRef,
}

#[derive(Default)]
pub struct MandelbrotControlSection {
    pub label_iterations: NodeRef,
    pub iters_input: InputNumberElement,
    pub hidden: NodeRef,
    pub ref_attr: NodeRef,
}

#[derive(Default)]
pub struct InputNumberElement {
    pub type_attr: NodeRef,
    pub min: NodeRef,
    pub id: NodeRef,
    pub ref_attr: NodeRef,
    pub onchange: NodeRef,
}

#[derive(Default)]
pub struct FooterSection {
    pub source_link: NodeRef,
    pub repo_link: NodeRef,
    pub crates_link: NodeRef,
    pub docs_link: NodeRef,
}
