#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct IconProps {
    #[props(default = "black".to_string())]
    stroke: String,
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "24px".to_string())]
    width: String,
    #[props(default = "24px".to_string())]
    height: String,
    class: Option<String>,
}

#[component]
pub fn TopDropdownArrow(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            height,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            width,
            path {
                "stroke-width": "2",
                "stroke-linejoin": "round",
                stroke: "#555462",
                "stroke-linecap": "round",
                fill: "#555462",
                d: "M11.9993 9.33341L6.66602 14.6667L17.3327 14.6667L11.9993 9.33341Z",
            }
        }
    }
}

#[component]
pub fn BottomDropdownArrow(width: String, height: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            height,
            fill: "none",
            path {
                d: "M12.0007 14.6666L17.334 9.33325L6.66732 9.33325L12.0007 14.6666Z",
                fill: "#555462",
                "stroke-width": "2",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                stroke: "#555462",
            }
        }
    }
}

#[component]
pub fn CalendarIcon(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            height,
            view_box: "0 0 28 28",
            width,
            path {
                d: "M22.4008 7.0002V6.3002C22.4008 5.1382 21.4628 4.2002 20.3008 4.2002C19.1388 4.2002 18.2008 5.1382 18.2008 6.3002V7.0002H9.80078V6.3002C9.80078 5.1382 8.86278 4.2002 7.70078 4.2002C6.53878 4.2002 5.60078 5.1382 5.60078 6.3002V7.0002C4.06078 7.0002 2.80078 8.2602 2.80078 9.8002V12.6002V14.0002V22.4002C2.80078 23.9402 4.06078 25.2002 5.60078 25.2002H22.4008C23.9408 25.2002 25.2008 23.9402 25.2008 22.4002V9.8002C25.2008 8.2602 23.9408 7.0002 22.4008 7.0002ZM19.6008 8.4002V7.0002V6.3002C19.6008 5.9082 19.9088 5.6002 20.3008 5.6002C20.6928 5.6002 21.0008 5.9082 21.0008 6.3002V7.0002V8.4002V9.1002C21.0008 9.4922 20.6928 9.8002 20.3008 9.8002C19.9088 9.8002 19.6008 9.4922 19.6008 9.1002V8.4002ZM7.00078 8.4002V7.0002V6.3002C7.00078 5.9082 7.30878 5.6002 7.70078 5.6002C8.09278 5.6002 8.40078 5.9082 8.40078 6.3002V7.0002V8.4002V9.1002C8.40078 9.4922 8.09278 9.8002 7.70078 9.8002C7.30878 9.8002 7.00078 9.4922 7.00078 9.1002V8.4002ZM23.8008 22.4002C23.8008 23.1702 23.1708 23.8002 22.4008 23.8002H5.60078C4.83078 23.8002 4.20078 23.1702 4.20078 22.4002V14.0002H23.8008V22.4002ZM23.8008 12.6002H4.20078V9.8002C4.20078 9.0302 4.83078 8.4002 5.60078 8.4002V9.1002C5.60078 10.2622 6.53878 11.2002 7.70078 11.2002C8.86278 11.2002 9.80078 10.2622 9.80078 9.1002V8.4002H18.2008V9.1002C18.2008 10.2622 19.1388 11.2002 20.3008 11.2002C21.4628 11.2002 22.4008 10.2622 22.4008 9.1002V8.4002C23.1708 8.4002 23.8008 9.0302 23.8008 9.8002V12.6002Z",
                fill: "#7C8292",
            }
            path {
                d: "M7.7 18.2001C8.092 18.2001 8.4 17.8921 8.4 17.5001V16.1001C8.4 15.7081 8.092 15.4001 7.7 15.4001C7.308 15.4001 7 15.7081 7 16.1001V17.5001C7 17.8921 7.308 18.2001 7.7 18.2001Z",
                fill: "#7C8292",
            }
            path {
                fill: "#7C8292",
                d: "M11.9012 18.2001C12.2932 18.2001 12.6012 17.8921 12.6012 17.5001V16.1001C12.6012 15.7081 12.2932 15.4001 11.9012 15.4001C11.5092 15.4001 11.2012 15.7081 11.2012 16.1001V17.5001C11.2012 17.8921 11.5092 18.2001 11.9012 18.2001Z",
            }
            path {
                d: "M16.1004 18.2001C16.4924 18.2001 16.8004 17.8921 16.8004 17.5001V16.1001C16.8004 15.7081 16.4924 15.4001 16.1004 15.4001C15.7084 15.4001 15.4004 15.7081 15.4004 16.1001V17.5001C15.4004 17.8921 15.7084 18.2001 16.1004 18.2001Z",
                fill: "#7C8292",
            }
            path {
                d: "M20.3016 18.2001C20.6936 18.2001 21.0016 17.8921 21.0016 17.5001V16.1001C21.0016 15.7081 20.6936 15.4001 20.3016 15.4001C19.9096 15.4001 19.6016 15.7081 19.6016 16.1001V17.5001C19.6016 17.8921 19.9096 18.2001 20.3016 18.2001Z",
                fill: "#7C8292",
            }
            path {
                fill: "#7C8292",
                d: "M7.7 22.4001C8.092 22.4001 8.4 22.0921 8.4 21.7001V20.3001C8.4 19.9081 8.092 19.6001 7.7 19.6001C7.308 19.6001 7 19.9081 7 20.3001V21.7001C7 22.0921 7.308 22.4001 7.7 22.4001Z",
            }
            path {
                d: "M11.9012 22.4001C12.2932 22.4001 12.6012 22.0921 12.6012 21.7001V20.3001C12.6012 19.9081 12.2932 19.6001 11.9012 19.6001C11.5092 19.6001 11.2012 19.9081 11.2012 20.3001V21.7001C11.2012 22.0921 11.5092 22.4001 11.9012 22.4001Z",
                fill: "#7C8292",
            }
            path {
                d: "M16.1004 22.4001C16.4924 22.4001 16.8004 22.0921 16.8004 21.7001V20.3001C16.8004 19.9081 16.4924 19.6001 16.1004 19.6001C15.7084 19.6001 15.4004 19.9081 15.4004 20.3001V21.7001C15.4004 22.0921 15.7084 22.4001 16.1004 22.4001Z",
                fill: "#7C8292",
            }
            path {
                fill: "#7C8292",
                d: "M20.3016 22.4001C20.6936 22.4001 21.0016 22.0921 21.0016 21.7001V20.3001C21.0016 19.9081 20.6936 19.6001 20.3016 19.6001C19.9096 19.6001 19.6016 19.9081 19.6016 20.3001V21.7001C19.6016 22.0921 19.9096 22.4001 20.3016 22.4001Z",
            }
        }
    }
}

#[component]
pub fn Trash(width: String, height: String) -> Element {
    rsx! {
        svg {
            height,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            width,
            fill: "none",
            path {
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                d: "M10 12V17",
                stroke: "#7C8292",
                "stroke-width": "2",
            }
            path {
                stroke: "#7C8292",
                d: "M14 12V17",
                "stroke-linejoin": "round",
                "stroke-width": "2",
                "stroke-linecap": "round",
            }
            path {
                "stroke-linecap": "round",
                d: "M4 7H20",
                stroke: "#7C8292",
                "stroke-linejoin": "round",
                "stroke-width": "2",
            }
            path {
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                d: "M6 10V18C6 19.6569 7.34315 21 9 21H15C16.6569 21 18 19.6569 18 18V10",
                "stroke-width": "2",
                stroke: "#7C8292",
            }
            path {
                "stroke-width": "2",
                d: "M9 5C9 3.89543 9.89543 3 11 3H13C14.1046 3 15 3.89543 15 5V7H9V5Z",
                "stroke-linecap": "round",
                stroke: "#7C8292",
                "stroke-linejoin": "round",
            }
        }
    }
}

#[component]
pub fn RowMenuDial(width: String, height: String) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width,
            fill: "none",
            height,
            view_box: "0 0 24 24",
            path {
                d: "M6 8.5C6 9.05228 5.55228 9.5 5 9.5C4.44772 9.5 4 9.05228 4 8.5C4 7.94772 4.44772 7.5 5 7.5C5.55228 7.5 6 7.94772 6 8.5Z",
                fill: "#B4B4B4",
            }
            path {
                d: "M6 15.5C6 16.0523 5.55228 16.5 5 16.5C4.44772 16.5 4 16.0523 4 15.5C4 14.9477 4.44772 14.5 5 14.5C5.55228 14.5 6 14.9477 6 15.5Z",
                fill: "#B4B4B4",
            }
            path {
                fill: "#B4B4B4",
                d: "M13 8.5C13 9.05228 12.5523 9.5 12 9.5C11.4477 9.5 11 9.05228 11 8.5C11 7.94772 11.4477 7.5 12 7.5C12.5523 7.5 13 7.94772 13 8.5Z",
            }
            path {
                d: "M13 15.5C13 16.0523 12.5523 16.5 12 16.5C11.4477 16.5 11 16.0523 11 15.5C11 14.9477 11.4477 14.5 12 14.5C12.5523 14.5 13 14.9477 13 15.5Z",
                fill: "#B4B4B4",
            }
            path {
                d: "M20 8.5C20 9.05228 19.5523 9.5 19 9.5C18.4477 9.5 18 9.05228 18 8.5C18 7.94772 18.4477 7.5 19 7.5C19.5523 7.5 20 7.94772 20 8.5Z",
                fill: "#B4B4B4",
            }
            path {
                d: "M20 15.5C20 16.0523 19.5523 16.5 19 16.5C18.4477 16.5 18 16.0523 18 15.5C18 14.9477 18.4477 14.5 19 14.5C19.5523 14.5 20 14.9477 20 15.5Z",
                fill: "#B4B4B4",
            }
            path {
                stroke: "#B4B4B4",
                "stroke-linecap": "round",
                d: "M6 8.5C6 9.05228 5.55228 9.5 5 9.5C4.44772 9.5 4 9.05228 4 8.5C4 7.94772 4.44772 7.5 5 7.5C5.55228 7.5 6 7.94772 6 8.5Z",
                "stroke-linejoin": "round",
            }
            path {
                "stroke-linecap": "round",
                stroke: "#B4B4B4",
                "stroke-linejoin": "round",
                d: "M6 15.5C6 16.0523 5.55228 16.5 5 16.5C4.44772 16.5 4 16.0523 4 15.5C4 14.9477 4.44772 14.5 5 14.5C5.55228 14.5 6 14.9477 6 15.5Z",
            }
            path {
                d: "M13 8.5C13 9.05228 12.5523 9.5 12 9.5C11.4477 9.5 11 9.05228 11 8.5C11 7.94772 11.4477 7.5 12 7.5C12.5523 7.5 13 7.94772 13 8.5Z",
                "stroke-linecap": "round",
                stroke: "#B4B4B4",
                "stroke-linejoin": "round",
            }
            path {
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                d: "M13 15.5C13 16.0523 12.5523 16.5 12 16.5C11.4477 16.5 11 16.0523 11 15.5C11 14.9477 11.4477 14.5 12 14.5C12.5523 14.5 13 14.9477 13 15.5Z",
                stroke: "#B4B4B4",
            }
            path {
                stroke: "#B4B4B4",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                d: "M20 8.5C20 9.05228 19.5523 9.5 19 9.5C18.4477 9.5 18 9.05228 18 8.5C18 7.94772 18.4477 7.5 19 7.5C19.5523 7.5 20 7.94772 20 8.5Z",
            }
            path {
                stroke: "#B4B4B4",
                d: "M20 15.5C20 16.0523 19.5523 16.5 19 16.5C18.4477 16.5 18 16.0523 18 15.5C18 14.9477 18.4477 14.5 19 14.5C19.5523 14.5 20 14.9477 20 15.5Z",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
        }
    }
}

#[component]
pub fn MenuDial(width: String, height: String) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width,
            height,
            fill: "none",
            view_box: "0 0 24 24",
            path {
                d: "M8.5 18C9.05228 18 9.5 18.4477 9.5 19C9.5 19.5523 9.05228 20 8.5 20C7.94772 20 7.5 19.5523 7.5 19C7.5 18.4477 7.94772 18 8.5 18Z",
                fill: "#B4B4B4",
            }
            path {
                fill: "#B4B4B4",
                d: "M15.5 18C16.0523 18 16.5 18.4477 16.5 19C16.5 19.5523 16.0523 20 15.5 20C14.9477 20 14.5 19.5523 14.5 19C14.5 18.4477 14.9477 18 15.5 18Z",
            }
            path {
                fill: "#B4B4B4",
                d: "M8.5 11C9.05228 11 9.5 11.4477 9.5 12C9.5 12.5523 9.05228 13 8.5 13C7.94771 13 7.5 12.5523 7.5 12C7.5 11.4477 7.94771 11 8.5 11Z",
            }
            path {
                fill: "#B4B4B4",
                d: "M15.5 11C16.0523 11 16.5 11.4477 16.5 12C16.5 12.5523 16.0523 13 15.5 13C14.9477 13 14.5 12.5523 14.5 12C14.5 11.4477 14.9477 11 15.5 11Z",
            }
            path {
                fill: "#B4B4B4",
                d: "M8.5 4C9.05228 4 9.5 4.44771 9.5 5C9.5 5.55229 9.05228 6 8.5 6C7.94771 6 7.5 5.55229 7.5 5C7.5 4.44771 7.94771 4 8.5 4Z",
            }
            path {
                d: "M15.5 4C16.0523 4 16.5 4.44771 16.5 5C16.5 5.55228 16.0523 6 15.5 6C14.9477 6 14.5 5.55228 14.5 5C14.5 4.44771 14.9477 4 15.5 4Z",
                fill: "#B4B4B4",
            }
            path {
                stroke: "#B4B4B4",
                "stroke-linecap": "round",
                d: "M8.5 18C9.05228 18 9.5 18.4477 9.5 19C9.5 19.5523 9.05228 20 8.5 20C7.94772 20 7.5 19.5523 7.5 19C7.5 18.4477 7.94772 18 8.5 18Z",
                "stroke-linejoin": "round",
            }
            path {
                d: "M15.5 18C16.0523 18 16.5 18.4477 16.5 19C16.5 19.5523 16.0523 20 15.5 20C14.9477 20 14.5 19.5523 14.5 19C14.5 18.4477 14.9477 18 15.5 18Z",
                "stroke-linecap": "round",
                stroke: "#B4B4B4",
                "stroke-linejoin": "round",
            }
            path {
                d: "M8.5 11C9.05228 11 9.5 11.4477 9.5 12C9.5 12.5523 9.05228 13 8.5 13C7.94771 13 7.5 12.5523 7.5 12C7.5 11.4477 7.94771 11 8.5 11Z",
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                stroke: "#B4B4B4",
            }
            path {
                "stroke-linecap": "round",
                stroke: "#B4B4B4",
                d: "M15.5 11C16.0523 11 16.5 11.4477 16.5 12C16.5 12.5523 16.0523 13 15.5 13C14.9477 13 14.5 12.5523 14.5 12C14.5 11.4477 14.9477 11 15.5 11Z",
                "stroke-linejoin": "round",
            }
            path {
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                stroke: "#B4B4B4",
                d: "M8.5 4C9.05228 4 9.5 4.44771 9.5 5C9.5 5.55229 9.05228 6 8.5 6C7.94771 6 7.5 5.55229 7.5 5C7.5 4.44771 7.94771 4 8.5 4Z",
            }
            path {
                d: "M15.5 4C16.0523 4 16.5 4.44771 16.5 5C16.5 5.55228 16.0523 6 15.5 6C14.9477 6 14.5 5.55228 14.5 5C14.5 4.44771 14.9477 4 15.5 4Z",
                stroke: "#B4B4B4",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
        }
    }
}

pub fn Cancel(props: IconProps) -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 23",
            fill: "{props.fill}",
            xmlns: "http://www.w3.org/2000/svg",
            width: "{props.width}",
            height: "{props.height}",
            path {
                d: "M23.5 11.5C23.5 17.5553 18.3715 22.5 12 22.5C5.62846 22.5 0.5 17.5553 0.5 11.5C0.5 5.44471 5.62846 0.5 12 0.5C18.3715 0.5 23.5 5.44471 23.5 11.5Z",
                fill: "#F7F7F7",
                stroke: "#AEAEAE",
            }
            path {
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                d: "M8 15.8645L11.9323 11.9323L15.8645 15.8645M15.8645 8L11.9315 11.9323L8 8",
                "stroke-width": "1.5",
                stroke: "{props.stroke}",
            }
        }
    }
}

#[component]
pub fn Clear(width: String, height: String) -> Element {
    rsx! {
        svg {
            width,
            height,
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                stroke: "white",
                "stroke-width": "1.5",
                d: "M9 9L15 15",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
            path {
                stroke: "white",
                "stroke-linecap": "round",
                "stroke-width": "1.5",
                d: "M15 9L9 15",
                "stroke-linejoin": "round",
            }
        }
    }
}

#[component]
pub fn Remove(
    width: String,
    height: String,
    #[props(default = "white".to_string())] fill: String,
) -> Element {
    rsx! {
        svg {
            view_box: "0 0 18 18",
            xmlns: "http://www.w3.org/2000/svg",
            height,
            fill: "none",
            width,
            path {
                "clip-rule": "evenodd",
                "fill-rule": "evenodd",
                fill,
                d: "M18 9C18 13.9706 13.9706 18 9 18C4.02944 18 0 13.9706 0 9C0 4.02944 4.02944 0 9 0C13.9706 0 18 4.02944 18 9ZM5.46967 5.46967C5.76256 5.17678 6.23744 5.17678 6.53033 5.46967L9 7.93934L11.4697 5.46967C11.7626 5.17678 12.2374 5.17678 12.5303 5.46967C12.8232 5.76256 12.8232 6.23744 12.5303 6.53033L10.0607 9L12.5303 11.4697C12.8232 11.7626 12.8232 12.2374 12.5303 12.5303C12.2374 12.8232 11.7626 12.8232 11.4697 12.5303L9 10.0607L6.53033 12.5303C6.23744 12.8232 5.76256 12.8232 5.46967 12.5303C5.17678 12.2374 5.17678 11.7626 5.46967 11.4697L7.93934 9L5.46967 6.53033C5.17678 6.23744 5.17678 5.76256 5.46967 5.46967Z",
            }
        }
    }
}

#[component]
pub fn Plus(
    width: String,
    height: String,
    #[props(default = "#35343F".to_string())] color: String,
) -> Element {
    rsx! {
        svg {
            view_box: "0 0 11 10",
            width,
            height,
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                "stroke-linecap": "round",
                d: "M1.5 5L5.5 5M5.5 5L9.5 5M5.5 5V1M5.5 5L5.5 9",
                "stroke-linejoin": "round",
                "stroke-width": "1.5",
                stroke: color,
            }
        }
    }
}

#[component]
pub fn ArrowLeft(
    width: String,
    height: String,
    #[props(default = "#9b9b9b".to_string())] color: String,
) -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "19",
            view_box: "0 0 10 19",
            width: "10",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M8.17639 0.724391L0.712384 8.59639C0.508384 8.81239 0.400391 9.08839 0.400391 9.38839C0.400391 9.68839 0.508384 9.97639 0.712384 10.1804L8.17639 18.0524C8.38039 18.2684 8.64439 18.3764 8.93239 18.3764C9.22039 18.3764 9.4844 18.2564 9.6884 18.0524C9.8924 17.8364 10.0004 17.5604 10.0004 17.2604C10.0004 16.9604 9.8924 16.6724 9.6884 16.4684L2.98038 9.38839L9.6884 2.30839C9.8924 2.09239 10.0004 1.81639 10.0004 1.51639C10.0004 1.21639 9.8924 0.928391 9.6884 0.724391C9.4844 0.508391 9.22039 0.400391 8.93239 0.400391C8.64439 0.400391 8.38039 0.520391 8.17639 0.724391Z",
                fill: "#B4B4B4",
            }
        }
    }
}

#[component]
pub fn AddUser(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            width: "24",
            xmlns: "http://www.w3.org/2000/svg",
            height: "24",
            view_box: "0 0 24 24",
            path {
                d: "M4 20V19C4 16.2386 6.23858 14 9 14H12.75M17.5355 13.9645V17.5M17.5355 17.5V21.0355M17.5355 17.5H21.0711M17.5355 17.5H14M15 7C15 9.20914 13.2091 11 11 11C8.79086 11 7 9.20914 7 7C7 4.79086 8.79086 3 11 3C13.2091 3 15 4.79086 15 7Z",
                "stroke-linecap": "round",
                stroke: "#AFC9FF",
                "stroke-width": "1.5",
                "stroke-linejoin": "round",
            }
        }
    }
}

#[component]
pub fn ArrowRight(
    width: String,
    height: String,
    #[props(default = "#9b9b9b".to_string())] color: String,
) -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "18",
            view_box: "0 0 10 18",
            width: "10",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M2.02479 17.6526L9.48879 9.78056C9.69279 9.56456 9.80078 9.28856 9.80078 8.98856C9.80078 8.68856 9.69279 8.40056 9.48879 8.19656L2.02479 0.324562C1.82079 0.108562 1.55678 0.000561614 1.26878 0.000561589C0.980782 0.000561564 0.716776 0.120562 0.512776 0.324562C0.308776 0.540562 0.200782 0.816562 0.200782 1.11656C0.200782 1.41656 0.308776 1.70456 0.512776 1.90856L7.22079 8.98856L0.512774 16.0686C0.308774 16.2846 0.200781 16.5606 0.200781 16.8606C0.200781 17.1606 0.308774 17.4486 0.512774 17.6526C0.716774 17.8686 0.98078 17.9766 1.26878 17.9766C1.55678 17.9766 1.82079 17.8566 2.02479 17.6526Z",
                fill: "#B4B4B4",
            }
        }
    }
}

#[component]
pub fn Expand(width: String, height: String) -> Element {
    rsx! {
        svg {
            "viewBox": "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            height,
            width,
            rect {
                width: "24",
                rx: "4",
                fill: "#7C8292",
                height: "24",
            }
            path {
                d: "M13.5 10.5L17.25 6.75M17.25 6.75L17.25 10.5M17.25 6.75L13.5 6.75",
                "stroke-width": "1.5",
                stroke: "white",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
            path {
                "stroke-linecap": "round",
                d: "M10.5 13.5L6.75 17.25M6.75 17.25V13.5M6.75 17.25H10.5",
                "stroke-width": "1.5",
                "stroke-linejoin": "round",
                stroke: "white",
            }
        }
    }
}

#[component]
pub fn Switch(width: String, height: String) -> Element {
    rsx! {
        svg {
            width,
            view_box: "0 0 19 18",
            height,
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            path {
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                stroke: "#9B9B9B",
                "stroke-width": "1.5",
                d: "M11 4.5L11 13.5L14 10.5",
            }
            path {
                "stroke-width": "1.5",
                "stroke-linejoin": "round",
                d: "M8 13.5L8 4.5L5 7.5",
                "stroke-linecap": "round",
                stroke: "#9B9B9B",
            }
        }
    }
}

#[component]
pub fn RowOption(width: String, height: String) -> Element {
    rsx! {
        svg {
            width,
            height,
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            rect {
                width: "24",
                height: "24",
                rx: "1.88235",
                fill: "white",
            }
            circle {
                fill: "#555462",
                cy: "12",
                cx: "5.80078",
                r: "1.5",
            }
            circle {
                cy: "12",
                r: "1.5",
                fill: "#555462",
                cx: "12",
            }
            circle {
                cy: "12",
                r: "1.5",
                cx: "18.2031",
                fill: "#555462",
            }
        }
    }
}

#[component]
pub fn ColOption(width: String, height: String) -> Element {
    rsx! {
        svg {
            height,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 40 40",
            fill: "none",
            width,
            rect {
                fill: "transparent",
                width: "40",
                rx: "4",
                height: "40",
            }
            circle {
                cx: "20",
                fill: "#555462",
                cy: "12",
                transform: "rotate(90 20 12)",
                r: "2",
            }
            circle {
                cy: "20",
                transform: "rotate(90 20 20)",
                r: "2",
                fill: "#555462",
                cx: "20",
            }
            circle {
                r: "2",
                fill: "#555462",
                cy: "28",
                cx: "20",
                transform: "rotate(90 20 28)",
            }
        }
    }
}

#[component]
pub fn Folder(width: String, height: String) -> Element {
    rsx! {
        svg {
            height,
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            width,
            path {
                "stroke-linejoin": "round",
                "stroke-width": "1.5",
                stroke: "#AFC9FF",
                d: "M12 19H5C3.89543 19 3 18.1046 3 17V7C3 5.89543 3.89543 5 5 5H9.58579C9.851 5 10.1054 5.10536 10.2929 5.29289L12 7H19C20.1046 7 21 7.89543 21 9V11",
                "stroke-linecap": "round",
            }
            path {
                stroke: "#AFC9FF",
                d: "M18 14V17M18 20V17M18 17H15M18 17H21",
                "stroke-width": "1.5",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
        }
    }
}

#[component]
pub fn Search(width: String, height: String, color: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 21 20",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            width,
            height,
            path {
                d: "M19.7959 19L15.0383 14.4439M15.0383 14.4439C15.8019 13.7126 16.4076 12.8444 16.8209 11.8889C17.2342 10.9335 17.4469 9.90942 17.4469 8.87523C17.4469 7.84104 17.2342 6.81697 16.8209 5.86151C16.4076 4.90604 15.8019 4.03788 15.0383 3.3066C14.2747 2.57532 13.3681 1.99523 12.3704 1.59947C11.3727 1.2037 10.3034 1 9.22344 1C8.14352 1 7.07418 1.2037 6.07646 1.59947C5.07875 1.99523 4.17221 2.57532 3.40859 3.3066C1.8664 4.78349 1 6.78659 1 8.87523C1 10.9639 1.8664 12.967 3.40859 14.4439C4.95078 15.9207 7.04244 16.7505 9.22344 16.7505C11.4044 16.7505 13.4961 15.9207 15.0383 14.4439Z",
                stroke: color,
                "stroke-width": "2",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
        }
    }
}

#[component]
pub fn Add(width: String, height: String, inner_color: String, color: String) -> Element {
    rsx! {
        svg {
            width: width.clone(),
            height,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 21 21",
            fill: "none",
            circle {
                cx: "10.5",
                cy: "10.5",
                r: "8",
                fill: inner_color,
            }
            path {
                d: "M10.0625 0C7.40378 0.0322598 4.86304 1.10277 2.98291 2.98291C1.10277 4.86304 0.0322598 7.40378 0 10.0625C0.0322598 12.7212 1.10277 15.262 2.98291 17.1421C4.86304 19.0222 7.40378 20.0927 10.0625 20.125C12.7212 20.0927 15.262 19.0222 17.1421 17.1421C19.0222 15.262 20.0927 12.7212 20.125 10.0625C20.0927 7.40378 19.0222 4.86304 17.1421 2.98291C15.262 1.10277 12.7212 0.0322598 10.0625 0ZM15.8125 10.7812H10.7812V15.8125H9.34375V10.7812H4.3125V9.34375H9.34375V4.3125H10.7812V9.34375H15.8125V10.7812Z",
                fill: color,
            }
        }
    }
}

#[component]
pub fn ModalCancel(width: String, height: String) -> Element {
    rsx! {
        svg {
            width,
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            height,
            view_box: "0 0 24 25",
            path {
                "stroke-linecap": "round",
                stroke: "#555462",
                d: "M8 8.5L16 16.5",
                "stroke-linejoin": "round",
                "stroke-width": "2",
            }
            path {
                "stroke-linejoin": "round",
                "stroke-width": "2",
                d: "M16 8.5L8 16.5",
                "stroke-linecap": "round",
                stroke: "#555462",
            }
        }
    }
}

#[component]
pub fn Close(
    width: String,
    height: String,
    color: String,
    border_color: String,
    button_color: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 23",
            width,
            height,
            fill: "none",
            path {
                stroke: border_color,
                d: "M23.5 11.5C23.5 17.5553 18.3715 22.5 12 22.5C5.62846 22.5 0.5 17.5553 0.5 11.5C0.5 5.44471 5.62846 0.5 12 0.5C18.3715 0.5 23.5 5.44471 23.5 11.5Z",
                fill: color,
            }
            path {
                "stroke-linejoin": "round",
                "stroke-width": "1.5",
                stroke: button_color,
                d: "M8 15.8645L11.9323 11.9323L15.8645 15.8645M15.8645 8L11.9315 11.9323L8 8",
                "stroke-linecap": "round",
            }
        }
    }
}

#[component]
pub fn Logout(width: String, height: String) -> Element {
    rsx! {
        svg {
            height,
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            width,
            fill: "none",
            path {
                stroke: "#9B9B9B",
                d: "M12.4987 13.75V15.8333C12.4987 16.7538 11.7525 17.5 10.832 17.5H4.9987C4.07822 17.5 3.33203 16.7538 3.33203 15.8333V4.16667C3.33203 3.24619 4.07822 2.5 4.9987 2.5H10.832C11.7525 2.5 12.4987 3.24619 12.4987 4.16667V6.71875M9.16536 10H17.4987M17.4987 10L15.4154 7.91667M17.4987 10L15.4154 12.0833",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                "stroke-width": "1.5",
            }
        }
    }
}

#[component]
pub fn BottomArrow(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 14 15",
            height,
            path {
                stroke: "#9BAAE4",
                d: "M11.082 6.07585L7.7058 9.45207C7.31528 9.8426 6.68212 9.8426 6.29159 9.45207L2.91536 6.07585",
                "stroke-width": "1.5",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
        }
    }
}

#[component]
pub fn Logo(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            height,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 31 32",
            width,
            path {
                fill: "#EBEFF5",
                d: "M11.0349 22.6203C17.1286 22.6203 22.0685 17.6803 22.0685 11.5866C22.0685 10.896 22.0051 10.2202 21.8837 9.56472C21.2282 9.44333 20.5524 9.37988 19.8618 9.37988C13.7681 9.37988 8.82812 14.3198 8.82812 20.4135C8.82812 21.1042 8.89158 21.78 9.01297 22.4354C9.66843 22.5568 10.3442 22.6203 11.0349 22.6203Z",
            }
            path {
                fill: "#85AEE2",
                d: "M9.01176 22.4352C3.88367 21.4855 0 16.9895 0 11.5864C0 5.49267 4.93994 0.552734 11.0337 0.552734C16.4368 0.552734 20.9328 4.4364 21.8825 9.5645C21.227 9.44311 20.5512 9.37966 19.8606 9.37966C13.7669 9.37966 8.82692 14.3196 8.82692 20.4133C8.82692 21.1039 8.89037 21.7797 9.01176 22.4352Z",
                "clip-rule": "evenodd",
                "fill-rule": "evenodd",
            }
            path {
                "clip-rule": "evenodd",
                fill: "#85AEE2",
                "fill-rule": "evenodd",
                d: "M11.0336 22.62C17.1273 22.62 22.0673 17.6801 22.0673 11.5863C22.0673 10.8957 22.0038 10.2199 21.8824 9.56445C27.0105 10.5141 30.8942 15.0102 30.8942 20.4133C30.8942 26.507 25.9543 31.4469 19.8605 31.4469C14.4574 31.4469 9.96142 27.5633 9.01172 22.4352C9.66719 22.5565 10.343 22.62 11.0336 22.62Z",
            }
        }
    }
}

#[component]
pub fn UploadFile(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            width,
            height,
            view_box: "0 0 43 42",
            g { "clip-path": "url(#clip0_226_63962)",
                path {
                    fill: "#CED9F9",
                    d: "M34.0194 3.12061H14.752V11.1106H38.1344V7.23402C38.1344 4.96567 36.2884 3.12061 34.0194 3.12061Z",
                }
                path {
                    d: "M23.1133 12.3403H0.578125V4.92636C0.578125 2.20972 2.7888 0 5.50641 0H12.7117C13.4279 0 14.1178 0.150925 14.7445 0.434509C15.62 0.828964 16.372 1.47913 16.8995 2.3286L23.1133 12.3403Z",
                    fill: "#1640C1",
                }
                path {
                    fill: "#2354E6",
                    d: "M42.5781 14.0001V37.8815C42.5781 40.1527 40.7292 42 38.457 42H4.69923C2.42703 42 0.578125 40.1527 0.578125 37.8815V9.88062H38.457C40.7292 9.88062 42.5781 11.7286 42.5781 14.0001Z",
                }
                path {
                    fill: "#1849D6",
                    d: "M42.5781 14.0001V37.8815C42.5781 40.1527 40.7292 42 38.457 42H21.5781V9.88062H38.457C40.7292 9.88062 42.5781 11.7286 42.5781 14.0001Z",
                }
                path {
                    d: "M32.6252 25.9398C32.6252 32.0322 27.669 36.9887 21.5772 36.9887C15.4855 36.9887 10.5293 32.0322 10.5293 25.9398C10.5293 19.8483 15.4855 14.8918 21.5772 14.8918C27.669 14.8918 32.6252 19.8483 32.6252 25.9398Z",
                    fill: "#E7ECFC",
                }
                path {
                    fill: "#CED9F9",
                    d: "M32.6261 25.9398C32.6261 32.0322 27.6699 36.9887 21.5781 36.9887V14.8918C27.6699 14.8918 32.6261 19.8483 32.6261 25.9398Z",
                }
                path {
                    fill: "#6C8DEF",
                    d: "M25.1393 26.0753C24.9089 26.2704 24.6266 26.3656 24.3469 26.3656C23.9967 26.3656 23.6487 26.2173 23.4051 25.9282L22.8088 25.2213V29.8494C22.8088 30.5287 22.2577 31.0799 21.5783 31.0799C20.899 31.0799 20.3479 30.5287 20.3479 29.8494V25.2213L19.7515 25.9282C19.3126 26.4476 18.5368 26.514 18.0174 26.0753C17.4983 25.6373 17.4316 24.8612 17.8696 24.3418L20.3053 21.4543C20.6228 21.0788 21.0862 20.8628 21.5783 20.8628C22.0705 20.8628 22.5339 21.0788 22.8514 21.4543L25.2871 24.3418C25.7251 24.8612 25.6584 25.6373 25.1393 26.0753Z",
                }
                path {
                    fill: "#3B67E9",
                    d: "M25.1391 26.0753C24.9087 26.2704 24.6264 26.3656 24.3467 26.3656C23.9964 26.3656 23.6485 26.2173 23.4049 25.9282L22.8086 25.2213V29.8494C22.8086 30.5287 22.2574 31.0799 21.5781 31.0799V20.8628C22.0703 20.8628 22.5337 21.0788 22.8512 21.4543L25.2868 24.3418C25.7249 24.8612 25.6582 25.6373 25.1391 26.0753Z",
                }
            }
            defs {
                clipPath { id: "clip0_226_63962",
                    rect {
                        transform: "translate(0.578125)",
                        fill: "white",
                        width: "42",
                        height: "42",
                    }
                }
            }
        }
    }
}

#[component]
pub fn CalendarLeftArrow() -> Element {
    rsx! {
        svg {
            width: "8",
            height: "14",
            view_box: "0 0 8 14",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M7 1L1 7L7 13",
                stroke: "#2a60d3",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}

#[component]
pub fn CalendarRightArrow() -> Element {
    rsx! {
        svg {
            width: "8",
            height: "14",
            view_box: "0 0 8 14",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M1 1L7 7L1 13",
                stroke: "#2a60d3",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}

#[component]
pub fn Checked(width: String, height: String) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width,
            view_box: "0 0 22 23",
            fill: "none",
            height,
            circle {
                stroke: "#2A60D3",
                cx: "11",
                cy: "11.5",
                r: "9.9",
                "stroke-width": "1.8",
            }
            circle {
                fill: "#2A60D3",
                cx: "11",
                r: "6.75",
                cy: "11.5",
            }
        }
    }
}

#[component]
pub fn UnChecked(width: String, height: String) -> Element {
    rsx! {
        svg {
            width,
            "viewBox": "0 0 22 23",
            fill: "none",
            height,
            xmlns: "http://www.w3.org/2000/svg",
            circle {
                stroke: "#B4B4B4",
                cy: "11.5",
                r: "9.9",
                cx: "11",
                "stroke-width": "1.8",
            }
        }
    }
}

#[component]
pub fn DiscussionUser(width: String, height: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            height,
            fill: "none",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            path {
                "stroke-linecap": "round",
                "stroke-width": "2",
                stroke: "#85AEE2",
                "stroke-linejoin": "round",
                d: "M3 19V18C3 15.7909 4.79086 14 7 14H11C13.2091 14 15 15.7909 15 18V19M15 11C16.6569 11 18 9.65685 18 8C18 6.34315 16.6569 5 15 5M21 19V18C21 15.7909 19.2091 14 17 14H16.5M12 8C12 9.65685 10.6569 11 9 11C7.34315 11 6 9.65685 6 8C6 6.34315 7.34315 5 9 5C10.6569 5 12 6.34315 12 8Z",
            }
        }
    }
}

#[component]
pub fn ClockIcon(width: String, height: String) -> Element {
    rsx! {
        svg {
            width,
            height,
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 28 28",
            circle {
                cx: "14",
                "stroke-width": "1.5",
                "stroke-linejoin": "round",
                stroke: "#7C8292",
                r: "10.5",
                cy: "14",
                "stroke-linecap": "round",
            }
            path {
                "stroke-linecap": "round",
                "stroke-width": "1.5",
                d: "M14 7.58325V13.9999L18.6667 16.3333",
                "stroke-linejoin": "round",
                stroke: "#7C8292",
            }
        }
    }
}

#[component]
pub fn SwitchOn(width: String, height: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 44 21",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            height,
            width,
            rect {
                width: "44",
                rx: "10",
                y: "0.5",
                height: "20",
                fill: "#2A60D3",
            }
            circle {
                cx: "34",
                r: "8",
                cy: "10.5",
                fill: "white",
            }
        }
    }
}

#[component]
pub fn SwitchOff(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            width,
            view_box: "0 0 44 21",
            xmlns: "http://www.w3.org/2000/svg",
            height,
            rect {
                rx: "10",
                height: "20",
                transform: "rotate(-180 44 20.5)",
                fill: "#B4B4B4",
                x: "44",
                width: "44",
                y: "20.5",
            }
            circle {
                fill: "white",
                cx: "10",
                cy: "10.5",
                transform: "rotate(-180 10 10.5)",
                r: "8",
            }
        }
    }
}

#[component]
pub fn Schedule(width: String, height: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            height,
            path {
                d: "M9 3H5C4.46957 3 3.96086 3.21071 3.58579 3.58579C3.21071 3.96086 3 4.46957 3 5V9M9 3H19C19.5304 3 20.0391 3.21071 20.4142 3.58579C20.7893 3.96086 21 4.46957 21 5V9M9 3V21M3 9V19C3 19.5304 3.21071 20.0391 3.58579 20.4142C3.96086 20.7893 4.46957 21 5 21H9M3 9H21M21 9V19C21 19.5304 20.7893 20.0391 20.4142 20.4142C20.0391 20.7893 19.5304 21 19 21H9",
                "stroke-linejoin": "round",
                stroke: "#AFC9FF",
                "stroke-width": "1.5",
                "stroke-linecap": "round",
            }
        }
    }
}

#[component]
pub fn Upload(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            "viewBox": "0 0 25 24",
            height,
            xmlns: "http://www.w3.org/2000/svg",
            width,
            path {
                "stroke-width": "1.5",
                d: "M19.5 15V17C19.5 18.1046 18.6046 19 17.5 19H7.5C6.39543 19 5.5 18.1046 5.5 17V15M12.5 15L12.5 5M12.5 5L14.5 7M12.5 5L10.5 7",
                "stroke-linejoin": "round",
                stroke: "#AFC9FF",
                "stroke-linecap": "round",
            }
        }
    }
}

#[component]
pub fn Edit(width: String, height: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 25 24",
            height,
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            width,
            path {
                stroke: "#AFC9FF",
                d: "M18.879 8.44975L11.9641 15.3647C11.6849 15.6439 11.3293 15.8342 10.9422 15.9117L8.00042 16.5L8.58877 13.5582C8.66621 13.1711 8.85652 12.8155 9.13571 12.5363L16.0506 5.62132M18.879 8.44975L20.2932 7.03553C20.6837 6.64501 20.6837 6.01184 20.2932 5.62132L18.879 4.20711C18.4885 3.81658 17.8553 3.81658 17.4648 4.20711L16.0506 5.62132M18.879 8.44975L16.0506 5.62132",
                "stroke-linecap": "round",
                "stroke-width": "1.5",
                "stroke-linejoin": "round",
            }
            path {
                d: "M5.5 20H19.5",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                "stroke-width": "1.5",
                stroke: "#AFC9FF",
            }
        }
    }
}

#[component]
pub fn Minus(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            view_box: "0 0 31 32",
            xmlns: "http://www.w3.org/2000/svg",
            width,
            height,
            path {
                fill: "#C0C0C0",
                d: "M9.3 14.45C8.88891 14.45 8.49467 14.6133 8.20398 14.904C7.9133 15.1947 7.75 15.5889 7.75 16C7.75 16.4111 7.9133 16.8053 8.20398 17.096C8.49467 17.3867 8.88891 17.55 9.3 17.55H21.7C22.1111 17.55 22.5053 17.3867 22.796 17.096C23.0867 16.8053 23.25 16.4111 23.25 16C23.25 15.5889 23.0867 15.1947 22.796 14.904C22.5053 14.6133 22.1111 14.45 21.7 14.45H9.3ZM15.5 0.5C11.3891 0.5 7.44666 2.13303 4.53984 5.03984C1.63303 7.94666 0 11.8891 0 16C0 20.1109 1.63303 24.0533 4.53984 26.9602C7.44666 29.867 11.3891 31.5 15.5 31.5C19.6109 31.5 23.5533 29.867 26.4602 26.9602C29.367 24.0533 31 20.1109 31 16C31 11.8891 29.367 7.94666 26.4602 5.03984C23.5533 2.13303 19.6109 0.5 15.5 0.5ZM3.1 16C3.1 12.7113 4.40642 9.55733 6.73188 7.23188C9.05733 4.90642 12.2113 3.6 15.5 3.6C18.7887 3.6 21.9427 4.90642 24.2681 7.23188C26.5936 9.55733 27.9 12.7113 27.9 16C27.9 19.2887 26.5936 22.4427 24.2681 24.7681C21.9427 27.0936 18.7887 28.4 15.5 28.4C12.2113 28.4 9.05733 27.0936 6.73188 24.7681C4.40642 22.4427 3.1 19.2887 3.1 16Z",
            }
        }
    }
}

#[component]
pub fn Message(width: String, height: String) -> Element {
    rsx! {
        svg {
            width,
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            height,
            view_box: "0 0 101 101",
            circle {
                cx: "50.5",
                cy: "50.75",
                fill: "#2A60D3",
                r: "50",
            }
            path {
                d: "M74.3168 33.4302H26.6875V68.0698H74.3168V33.4302Z",
                fill: "white",
            }
            path {
                fill: "#AFC9FF",
                d: "M69.554 63.7397H31.4503L26.6875 68.0696H74.3168L69.554 63.7397Z",
            }
            path {
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                stroke: "#182248",
                "stroke-width": "2.14886",
                "stroke-miterlimit": "10",
                d: "M74.3168 33.4302L50.5021 57.8944L26.6875 33.4302",
            }
            path {
                "stroke-width": "2.14886",
                "stroke-miterlimit": "10",
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                d: "M26.6875 68.0703L44.2237 51.833",
                stroke: "#182248",
            }
            path {
                stroke: "#182248",
                "stroke-linecap": "round",
                d: "M56.7773 51.833L74.3135 68.0703",
                "stroke-miterlimit": "10",
                "stroke-linejoin": "round",
                "stroke-width": "2.14886",
            }
            path {
                d: "M74.3168 33.4302H26.6875V68.0698H74.3168V33.4302Z",
                "stroke-linejoin": "round",
                stroke: "#182248",
                "stroke-linecap": "round",
                "stroke-width": "2.14886",
                "stroke-miterlimit": "10",
            }
        }
    }
}
