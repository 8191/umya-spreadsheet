// c:plotArea
use super::Layout;
use super::LineChart;
use super::Line3DChart;
use super::PieChart;
use super::DoughnutChart;
use super::ScatterChart;
use super::BarChart;
use super::Bar3DChart;
use super::RadarChart;
use super::BubbleChart;
use super::CategoryAxis;
use super::ValueAxis;
use super::SeriesAxis;
use super::ShapeProperties;
use super::Formula;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct PlotArea {
    layout: Layout,
    line_chart: Option<LineChart>,
    line_3d_chart: Option<Line3DChart>,
    pie_chart: Option<PieChart>,
    doughnut_chart: Option<DoughnutChart>,
    scatter_chart: Option<ScatterChart>,
    bar_chart: Option<BarChart>,
    bar_3d_chart: Option<Bar3DChart>,
    radar_chart: Option<RadarChart>,
    bubble_chart: Option<BubbleChart>,
    category_axis: Vec<CategoryAxis>,
    value_axis: Vec<ValueAxis>,
    series_axis: Vec<SeriesAxis>,
    shape_properties: Option<ShapeProperties>,
}
impl PlotArea {
    pub fn get_layout(&self)-> &Layout {
        &self.layout
    }

    pub fn get_layout_mut(&mut self)-> &mut Layout {
        &mut self.layout
    }

    pub fn set_layout(&mut self, value:Layout)-> &mut PlotArea {
        self.layout = value;
        self
    }

    pub fn get_line_chart(&self)-> &Option<LineChart> {
        &self.line_chart
    }

    pub fn get_line_chart_mut(&mut self)-> &mut Option<LineChart> {
        &mut self.line_chart
    }

    pub fn set_line_chart(&mut self, value:LineChart)-> &mut PlotArea {
        self.line_chart = Some(value);
        self
    }

    pub fn get_line_3d_chart(&self)-> &Option<Line3DChart> {
        &self.line_3d_chart
    }

    pub fn get_line_3d_chart_mut(&mut self)-> &mut Option<Line3DChart> {
        &mut self.line_3d_chart
    }

    pub fn set_line_3d_chart(&mut self, value:Line3DChart)-> &mut PlotArea {
        self.line_3d_chart = Some(value);
        self
    }

    pub fn get_pie_chart(&self)-> &Option<PieChart> {
        &self.pie_chart
    }

    pub fn get_pie_chart_mut(&mut self)-> &mut Option<PieChart> {
        &mut self.pie_chart
    }

    pub fn set_pie_chart(&mut self, value:PieChart)-> &mut PlotArea {
        self.pie_chart = Some(value);
        self
    }

    pub fn get_doughnut_chart(&self)-> &Option<DoughnutChart> {
        &self.doughnut_chart
    }

    pub fn get_doughnut_chart_mut(&mut self)-> &mut Option<DoughnutChart> {
        &mut self.doughnut_chart
    }

    pub fn set_doughnut_chart(&mut self, value:DoughnutChart)-> &mut PlotArea {
        self.doughnut_chart = Some(value);
        self
    }

    pub fn get_scatter_chart(&self)-> &Option<ScatterChart> {
        &self.scatter_chart
    }

    pub fn get_scatter_chart_mut(&mut self)-> &mut Option<ScatterChart> {
        &mut self.scatter_chart
    }

    pub fn set_scatter_chart(&mut self, value:ScatterChart)-> &mut PlotArea {
        self.scatter_chart = Some(value);
        self
    }

    pub fn get_bar_chart(&self)-> &Option<BarChart> {
        &self.bar_chart
    }

    pub fn get_bar_chart_mut(&mut self)-> &mut Option<BarChart> {
        &mut self.bar_chart
    }

    pub fn set_bar_chart(&mut self, value:BarChart)-> &mut PlotArea {
        self.bar_chart = Some(value);
        self
    }

    pub fn get_bar_3d_chart(&self)-> &Option<Bar3DChart> {
        &self.bar_3d_chart
    }

    pub fn get_bar_3d_chart_mut(&mut self)-> &mut Option<Bar3DChart> {
        &mut self.bar_3d_chart
    }

    pub fn set_bar_3d_chart(&mut self, value:Bar3DChart)-> &mut PlotArea {
        self.bar_3d_chart = Some(value);
        self
    }

    pub fn get_radar_chart(&self)-> &Option<RadarChart> {
        &self.radar_chart
    }

    pub fn get_radar_chart_mut(&mut self)-> &mut Option<RadarChart> {
        &mut self.radar_chart
    }

    pub fn set_radar_chart(&mut self, value:RadarChart)-> &mut PlotArea {
        self.radar_chart = Some(value);
        self
    }

    pub fn get_bubble_chart(&self)-> &Option<BubbleChart> {
        &self.bubble_chart
    }

    pub fn get_bubble_chart_mut(&mut self)-> &mut Option<BubbleChart> {
        &mut self.bubble_chart
    }

    pub fn set_bubble_chart(&mut self, value:BubbleChart)-> &mut PlotArea {
        self.bubble_chart = Some(value);
        self
    }

    pub fn get_category_axis(&self)-> &Vec<CategoryAxis> {
        &self.category_axis
    }

    pub fn get_category_axis_mut(&mut self)-> &mut Vec<CategoryAxis> {
        &mut self.category_axis
    }

    pub fn set_category_axis(&mut self, value:CategoryAxis)-> &mut PlotArea {
        self.category_axis.push(value);
        self
    }

    pub fn get_value_axis(&self)-> &Vec<ValueAxis> {
        &self.value_axis
    }

    pub fn get_value_axis_mut(&mut self)-> &mut Vec<ValueAxis> {
        &mut self.value_axis
    }

    pub fn set_value_axis(&mut self, value:ValueAxis)-> &mut PlotArea {
        self.value_axis.push(value);
        self
    }

    pub fn get_series_axis(&self)-> &Vec<SeriesAxis> {
        &self.series_axis
    }

    pub fn get_series_axis_mut(&mut self)-> &mut Vec<SeriesAxis> {
        &mut self.series_axis
    }

    pub fn set_series_axis(&mut self, value:SeriesAxis)-> &mut PlotArea {
        self.series_axis.push(value);
        self
    }

    pub fn get_shape_properties(&self)-> &Option<ShapeProperties> {
        &self.shape_properties
    }

    pub fn get_shape_properties_mut(&mut self)-> &Option<ShapeProperties> {
        &mut self.shape_properties
    }

    pub fn set_shape_properties(&mut self, value:ShapeProperties)-> &mut PlotArea {
        self.shape_properties = Some(value);
        self
    }

    pub fn get_formula_mut(&mut self)-> Vec<&mut Formula> {
        let mut result:Vec<&mut Formula> = Vec::default();
        match &mut self.line_chart {
            Some(v) => {
                for ser in v.get_area_chart_series_mut() {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.line_3d_chart {
            Some(v) => {
                for ser in v.get_area_chart_series_mut() {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.pie_chart {
            Some(v) => {
                for ser in v.get_area_chart_series_mut() {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.doughnut_chart {
            Some(v) => {
                for ser in v.get_area_chart_series_mut() {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.scatter_chart {
            Some(v) => {
                for ser in v.get_area_chart_series_mut() {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.bar_chart {
            Some(v) => {
                for ser in v.get_area_chart_series_mut() {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.bar_3d_chart {
            Some(v) => {
                for ser in v.get_area_chart_series_mut() {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.radar_chart {
            Some(v) => {
                for ser in v.get_area_chart_series_mut() {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        match &mut self.bubble_chart {
            Some(v) => {
                for ser in v.get_area_chart_series_mut() {
                    for formula in ser.get_formula_mut() {
                        result.push(formula);
                    }
                }
            }
            None => {}
        }
        result
    }

    pub(crate) fn is_support(&self) -> bool {
        match &self.line_chart {
            Some(_) => {return true;},
            None => {}
        }
        match &self.line_3d_chart {
            Some(_) => {return true;},
            None => {}
        }
        match &self.pie_chart {
            Some(_) => {return true;},
            None => {}
        }
        match &self.doughnut_chart {
            Some(_) => {return true;},
            None => {}
        }
        match &self.scatter_chart {
            Some(_) => {return true;},
            None => {}
        }
        match &self.bar_chart {
            Some(_) => {return true;},
            None => {}
        }
        match &self.bar_3d_chart {
            Some(_) => {return true;},
            None => {}
        }
        match &self.radar_chart {
            Some(_) => {return true;},
            None => {}
        }
        match &self.bubble_chart {
            Some(_) => {return true;},
            None => {}
        }
        false
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"c:layout" => {
                            self.layout.set_attributes(reader, e, false);
                        },
                        b"c:lineChart" => {
                            let mut obj = LineChart::default();
                            obj.set_attributes(reader, e);
                            self.set_line_chart(obj);
                        },
                        b"c:line3DChart" => {
                            let mut obj = Line3DChart::default();
                            obj.set_attributes(reader, e);
                            self.set_line_3d_chart(obj);
                        },
                        b"c:pieChart" => {
                            let mut obj = PieChart::default();
                            obj.set_attributes(reader, e);
                            self.set_pie_chart(obj);
                        },
                        b"c:doughnutChart" => {
                            let mut obj = DoughnutChart::default();
                            obj.set_attributes(reader, e);
                            self.set_doughnut_chart(obj);
                        },
                        b"c:scatterChart" => {
                            let mut obj = ScatterChart::default();
                            obj.set_attributes(reader, e);
                            self.set_scatter_chart(obj);
                        },
                        b"c:barChart" => {
                            let mut obj = BarChart::default();
                            obj.set_attributes(reader, e);
                            self.set_bar_chart(obj);
                        },
                        b"c:bar3DChart" => {
                            let mut obj = Bar3DChart::default();
                            obj.set_attributes(reader, e);
                            self.set_bar_3d_chart(obj);
                        },
                        b"c:radarChart" => {
                            let mut obj = RadarChart::default();
                            obj.set_attributes(reader, e);
                            self.set_radar_chart(obj);
                        },
                        b"c:bubbleChart" => {
                            let mut obj = BubbleChart::default();
                            obj.set_attributes(reader, e);
                            self.set_bubble_chart(obj);
                        },
                        b"c:catAx" => {
                            let mut obj = CategoryAxis::default();
                            obj.set_attributes(reader, e);
                            self.set_category_axis(obj);
                        },
                        b"c:valAx" => {
                            let mut obj = ValueAxis::default();
                            obj.set_attributes(reader, e);
                            self.set_value_axis(obj);
                        },
                        b"c:serAx" => {
                            let mut obj = SeriesAxis::default();
                            obj.set_attributes(reader, e);
                            self.set_series_axis(obj);
                        },
                        b"c:spPr" => {
                            let mut obj = ShapeProperties::default();
                            obj.set_attributes(reader, e);
                            self.set_shape_properties(obj);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:plotArea" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:plotArea"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:plotArea
        write_start_tag(writer, "c:plotArea", vec![], false);

        // c:layout
        &self.layout.write_to(writer);

        // c:lineChart
        match &self.line_chart {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:line3DChart
        match &self.line_3d_chart {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:pieChart
        match &self.pie_chart {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:doughnutChart
        match &self.doughnut_chart {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:scatterChart
        match &self.scatter_chart {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:barChart
        match &self.bar_chart {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:bar3DChart
        match &self.bar_3d_chart {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:radarChart
        match &self.radar_chart {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:bubbleChart
        match &self.bubble_chart {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        // c:catAx
        for v in &self.category_axis {
            v.write_to(writer);
        }

        // c:valAx
        for v in &self.value_axis {
            v.write_to(writer);
        }

        // c:serAx
        for v in &self.series_axis {
            v.write_to(writer);
        }
        
        // c:spPr
        match &self.shape_properties {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        write_end_tag(writer, "c:plotArea");
    }
}
