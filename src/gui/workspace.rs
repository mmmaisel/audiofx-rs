/******************************************************************************\
    wavehacker
    Copyright (C) 2023 Max Maisel

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
\******************************************************************************/
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;

use gtk4::gdk::Rectangle;
use gtk4::glib::{self, Object, ParamSpec, Properties, Value};
use gtk4::{
    Accessible, Actionable, Adjustment, Box, Buildable, ConstraintTarget,
    LayoutManager, ListBox, Orientable, Orientation, Overflow, Scrollable,
    ScrollablePolicy, SelectionMode, Viewport, Widget,
};

use std::cell::RefCell;

use super::widgets::{AudioTrack, Timeline};

#[derive(Default)]
struct WorkspaceLayoutImpl {}

#[glib::object_subclass]
impl ObjectSubclass for WorkspaceLayoutImpl {
    const NAME: &'static str = "WorkspaceLayout";
    type Type = WorkspaceLayout;
    type ParentType = LayoutManager;
}

impl ObjectImpl for WorkspaceLayoutImpl {}

impl LayoutManagerImpl for WorkspaceLayoutImpl {
    fn allocate(
        &self,
        widget: &Widget,
        width: i32,
        height: i32,
        baseline: i32,
    ) {
        let wsobj = widget.clone().downcast::<WavehackerWorkspace>().unwrap();
        let ws = WavehackerWorkspaceImpl::from_obj(&wsobj);

        let hy = ws.timeline.borrow().measure(Orientation::Vertical, -1);

        ws.timeline
            .borrow()
            .size_allocate(&Rectangle::new(0, 0, width, hy.1), baseline);

        ws.viewport.borrow().size_allocate(
            &Rectangle::new(0, hy.1, width, height - hy.1),
            baseline,
        );
    }

    fn measure(
        &self,
        widget: &Widget,
        orientation: Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32) {
        let wsobj = widget.clone().downcast::<WavehackerWorkspace>().unwrap();
        let ws = WavehackerWorkspaceImpl::from_obj(&wsobj);
        let vp_meas = ws.viewport.borrow().measure(orientation, for_size);
        let hd_meas = ws.timeline.borrow().measure(orientation, for_size);
        (vp_meas.0 + hd_meas.0, vp_meas.1 + hd_meas.1, -1, -1)
    }
}

glib::wrapper! {
    struct WorkspaceLayout(ObjectSubclass<WorkspaceLayoutImpl>)
        @extends LayoutManager;
}

impl WorkspaceLayout {
    fn new() -> Self {
        Object::builder().build()
    }
}

#[derive(Debug, Properties)]
#[properties(wrapper_type = WavehackerWorkspace)]
pub struct WavehackerWorkspaceImpl {
    #[property(get, set, nullable, override_interface = Scrollable)]
    hadjustment: RefCell<Option<Adjustment>>,
    #[property(get, set, override_interface = Scrollable)]
    hscroll_policy: RefCell<ScrollablePolicy>,
    #[property(get, set, nullable, override_interface = Scrollable)]
    vadjustment: RefCell<Option<Adjustment>>,
    #[property(get, set, override_interface = Scrollable)]
    vscroll_policy: RefCell<ScrollablePolicy>,

    pub timeline: RefCell<Timeline>,
    pub viewport: RefCell<Viewport>,
}

impl Default for WavehackerWorkspaceImpl {
    fn default() -> Self {
        Self {
            hadjustment: None.into(),
            hscroll_policy: ScrollablePolicy::Natural.into(),
            vadjustment: None.into(),
            vscroll_policy: ScrollablePolicy::Natural.into(),
            timeline: Timeline::default().into(),
            viewport: Viewport::default().into(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for WavehackerWorkspaceImpl {
    const NAME: &'static str = "WavehackerWorkspace";
    type Type = WavehackerWorkspace;
    type ParentType = Box;
    type Interfaces = (Scrollable,);
}

impl ObjectImpl for WavehackerWorkspaceImpl {
    fn properties() -> &'static [ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
        self.derived_property(id, pspec)
    }

    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        let timeline = self.timeline.borrow();
        let viewport = self.viewport.borrow();

        obj.set_spacing(4);
        obj.set_orientation(Orientation::Vertical);
        obj.set_layout_manager(Some(WorkspaceLayout::new()));

        let op_list = ListBox::builder()
            .selection_mode(SelectionMode::None)
            .show_separators(true)
            .overflow(Overflow::Visible)
            .hexpand(true)
            .vexpand(true)
            .build();

        for i in 0..5 {
            let track = AudioTrack::default();
            obj.bind_property(
                "hadjustment",
                &track.waveform().clone(),
                "hadjustment",
            )
            .sync_create()
            .build();
            obj.bind_property(
                "hscroll-policy",
                &track.waveform().clone(),
                "hscroll-policy",
            )
            .sync_create()
            .build();
            op_list.append(&track);
        }

        viewport.set_child(Some(&op_list));

        // TODO: add player widgets

        obj.bind_property("hadjustment", &timeline.clone(), "hadjustment")
            .sync_create()
            .build();
        obj.bind_property(
            "hscroll-policy",
            &timeline.clone(),
            "hscroll-policy",
        )
        .sync_create()
        .build();

        obj.bind_property("vadjustment", &viewport.clone(), "vadjustment")
            .sync_create()
            .build();
        obj.bind_property(
            "vscroll-policy",
            &viewport.clone(),
            "vscroll-policy",
        )
        .sync_create()
        .build();

        self.obj().append(&self.timeline.borrow().clone());
        self.obj().append(&self.viewport.borrow().clone());
    }
}

impl BoxImpl for WavehackerWorkspaceImpl {}
impl ScrollableImpl for WavehackerWorkspaceImpl {}
impl WidgetImpl for WavehackerWorkspaceImpl {}

glib::wrapper! {
    pub struct WavehackerWorkspace(ObjectSubclass<WavehackerWorkspaceImpl>)
        @extends Box, Widget,
        @implements Accessible, Actionable, Buildable, ConstraintTarget,
            Orientable, Scrollable;
}

impl WavehackerWorkspace {}

impl Default for WavehackerWorkspace {
    fn default() -> Self {
        Object::builder().build()
    }
}
