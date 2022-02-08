
/home/diego/Software/plots/src/lib.rs,9609
pub enum PlotError {PlotError62,1159
pub mod json {json76,1364
    pub struct Design {Design84,1562
        fn default() -> Self {default92,1743
    pub struct Layout {Layout106,2187
        fn default() -> Self {default115,2404
    pub struct Scale {Scale127,2703
        fn default() -> Self {default140,3009
    pub struct Map {Map156,3450
    pub struct Mapping {Mapping168,3719
    pub struct Plot {Plot200,4503
    pub struct Panel {Panel209,4740
mod xml {xml220,5041
pub enum GroupSplit {GroupSplit225,5106
    type Err = ();Err238,5275
    fn from_str(s : &str) -> Result<Self, ()> {from_str240,5295
fn n_plots_for_split(split : &GroupSplit) -> usize {n_plots_for_split256,5883
pub enum LayoutProperty {LayoutProperty265,6197
pub enum DesignProperty {DesignProperty273,6331
pub enum ScaleProperty {ScaleProperty280,6449
pub enum LineProperty {LineProperty292,6646
pub enum ScatterProperty {ScatterProperty300,6756
pub enum TextProperty {TextProperty307,6855
pub enum IntervalProperty {IntervalProperty315,6975
pub enum MappingProperty {MappingProperty327,7221
pub enum ScaleMode {ScaleMode334,7360
pub enum PlotProperty {PlotProperty340,7446
pub enum GroupProperty {GroupProperty348,7593
pub struct Panel {Panel367,8336
    fn default() -> Self {default396,8707
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {fmt412,9059
pub enum Orientation {Orientation428,9481
fn update_dims_from_env(dims : &mut (usize, usize)) {update_dims_from_env433,9536
    pub fn dimensions(mut self, w : u32, h : u32) -> Self {dimensions444,9816
    pub fn single(p1 : Plot) -> Self {single451,10006
    pub fn pair(orientation : Orientation, p1 : Plot, p2 : Plot) -> Self {pair459,10262
    pub fn update(&mut self, prop : GroupProperty) {update472,10727
    pub fn to_json(&self) -> String {to_json497,11963
    pub fn new() -> Self {new510,12439
    pub fn new_from_json(json : &str) -> Result<Self, String> {new_from_json514,12500
    pub fn update_mapping_with_adjustment(&mut self, active : usize, key : &str, data : Vec<Vec<f64>>, adj : Adjustment) {update_mapping_with_adjustment646,18093
    pub fn adjust_scales(&mut self, active : usize, adj_x : Adjustment, adj_y : Adjustment) {adjust_scales654,18465
    pub fn adjust_scales(&mut self) {adjust_scales662,18941
    pub fn clear_all_data(&mut self) {clear_all_data702,20526
    pub fn png(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {png708,20661
    pub fn html_img_tag(&mut self) -> Result<String, Box<dyn Error>> {html_img_tag721,21118
    pub fn svg(&mut self) -> Result<String, Box<dyn Error>> {svg726,21310
    pub fn show_with_eog(&mut self) -> Result<(), Box<dyn Error>> {show_with_eog754,22146
    pub fn show_with_app(&mut self, app : &str) -> Result<(), Box<dyn Error>> {show_with_app761,22425
    pub fn draw_to_file(&mut self, path : &str) -> Result<(), String> {draw_to_file774,22872
    pub fn size(&self) -> usize {size825,25264
    pub fn draw_to_context(draw_to_context834,25607
    pub fn update_mapping(&mut self, ix : usize, id : &str, data : &Vec<Vec<f64>>) -> Result<(), Box<dyn Error>> {update_mapping1086,36593
    pub fn update_mapping_text(&mut self, ix : usize, id : &str, text : &Vec<String>) -> Result<(), Box<dyn Error>> {update_mapping_text1091,36827
    pub fn update_mapping_columns(&mut self, ix : usize, id : &str, cols : Vec<String>) -> Result<(), Box<dyn Error>> {update_mapping_columns1095,37005
    pub fn update_source(&mut self, ix : usize, id : &str, source : String) -> Result<(), Box<dyn Error>> {update_source1099,37188
    pub fn ordered_col_names(&self, ix : usize, id : &str) -> Vec<(String, String)> {ordered_col_names1114,37686
    pub fn scale_info(&self, ix : usize, scale : &str) -> HashMap<String, String> {scale_info1124,38000
    pub fn design_info(&self) -> HashMap<String, String> {design_info1128,38132
    pub fn mapping_info(&self, ix : usize) -> Vec<(String, String, HashMap<String,String>)> {mapping_info1132,38232
    pub fn group_split(&self) -> GroupSplit {group_split1136,38371
    pub fn aspect_ratio(&self) -> (f64, f64) {aspect_ratio1140,38451
    pub fn data_limits(&self, ix : usize) -> Option<((f64, f64), (f64, f64))> {data_limits1144,38542
    pub fn set_aspect_ratio(&mut self, horiz : Option<f64>, vert : Option<f64>) {set_aspect_ratio1148,38670
    pub fn n_mappings(&self) -> Vec<usize> {n_mappings1181,39795
    pub fn n_plots(&self) -> usize {n_plots1186,39933
    pub fn view_all_sources(&self) -> Vec<String> {view_all_sources1190,40002
    pub fn view_grouped_sources(&self) -> Option<String> {view_grouped_sources1194,40140
    pub fn view_dimensions(&self) -> (u32, u32) {view_dimensions1207,40498
pub struct Plot {Plot1214,40643
    fn default() -> Self {default1224,40822
pub enum PlotError {PlotError1235,41155
    pub fn new() -> Self {new1243,41295
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {fmt1250,41398
    fn show(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {show1266,41812
    fn modality(&self) -> showable::Modality {modality1270,41935
    fn show(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {show1276,42055
    fn modality(&self) -> showable::Modality {modality1280,42179
    pub fn dimensions(mut self, w : u32, h : u32) -> Self {dimensions1291,42560
    pub fn wrap(&self) -> Panel {wrap1297,42729
    pub fn svg(&self) -> String {svg1301,42806
    pub fn draw_to_file(&self, path : &str) -> Result<(), String> {draw_to_file1305,42882
    pub fn scale_x(mut self, scale : Scale) -> Self {scale_x1309,42996
    pub fn scale_y(mut self, scale : Scale) -> Self {scale_y1315,43124
    pub fn draw(mut self, map : impl Mapping + 'static) -> Self {draw1321,43252
    pub fn update(&mut self, prop : PlotProperty) {update1327,43431
    pub fn adjust_scales(&mut self) {adjust_scales1344,43975
    pub fn new_from_json(mut rep : json::Plot) -> Result<Plot, Box<dyn Error>> {new_from_json1371,45261
    pub fn new() -> Self {new1413,46417
    fn draw_plot(&mut self, ctx: &Context, design : &PlotDesign, w : i32, h : i32) {draw_plot1436,47057
    pub fn max_data_limits(&self) -> Option<((f64, f64), (f64, f64))> {max_data_limits1452,47617
    pub fn freeze_at_mapping(&mut self, _mapping : &str) -> Result<(),()> {freeze_at_mapping1467,48420
    pub fn unfreeze(&mut self) {unfreeze1473,48579
    fn read_grid_segment(read_grid_segment1480,48765
    pub fn reload_mappings(&mut self) -> Result<(),String> {reload_mappings1498,49531
    pub fn reload_layout_node(&mut self /*, node : Node*/ ) -> Result<(), Box<dyn Error>> {reload_layout_node1535,51469
    pub fn add_mapping(add_mapping1576,53179
    fn accomodate_dimension(accomodate_dimension1699,58440
    pub fn update_mapping(update_mapping1740,59556
    pub fn update_mapping_text(update_mapping_text1796,62062
    pub fn update_layout(&mut self, property : &str, value : &str) -> Result<(), String> {update_layout1836,63444
    pub fn clear_all_data(&mut self) {clear_all_data1892,66266
    fn draw_background(&self, ctx : &Context, design : &PlotDesign) {draw_background1937,67925
    fn draw_grid_line(draw_grid_line1952,68419
    fn draw_grid_value(draw_grid_value1979,69344
    pub fn steps_to_labels(steps_to_labels2002,69853
    fn get_max_extent(get_max_extent2011,70058
    fn draw_grid(&self, ctx : &Context, design : &PlotDesign) {draw_grid2029,70456
    fn draw_scale_names(&self, ctx : &Context, design : &PlotDesign) {draw_scale_names2093,73788
    pub fn mapping_info(&self) -> Vec<(String, String, HashMap<String,String>)> {mapping_info2130,74749
    pub fn mapping_column_names(&self, id : &str) -> Vec<(String, String)> {mapping_column_names2139,75060
    pub fn scale_info(&self, scale : &str) -> HashMap<String, String> {scale_info2147,75332
    pub fn update_mapping_columns(update_mapping_columns2155,75557
    pub fn update_source(update_source2173,76112
    pub fn view_sources(&self) -> Vec<String> {view_sources2186,76481
    pub fn children_as_hash(children_as_hash2237,77771
    pub fn edit_node_with_hash(edit_node_with_hash2274,79051
pub extern "C" fn interactive(engine : &mut interactive::Engine) {interactive2306,80334
    extern "C" fn module() -> Box<interactive::Module> {module2317,80703
    extern "C" fn interactive(engine : &mut interactive::Engine) {interactive2329,81144
    fn fields(engine : &mut interactive::Engine) {fields2365,82920
extern "C" fn reg_methods(engine : &mut interactive::Engine) /*-> Box<interactive::Engine>*/ {reg_methods2378,83303
    extern "C" fn interactive(engine : &mut interactive::Engine) -> Box<interactive::RegistrationInfo> {interactive2412,84533
    extern "C" fn interactive(engine : &mut interactive::Engine) {interactive2435,85400
    const NAME: &'static str = "PlotView";NAME2461,85982
    type ParentType = gtk::DrawingArea;ParentType2462,86025
    type Class = subclass::simple::ClassStruct<Self>;Class2469,86429
    type Instance = subclass::simple::InstanceStruct<Self>;Instance2473,86656
    fn class_init(klass: &mut Self::Class) {class_init2477,86747
    fn new() -> Self {new2481,86846
    const NAME: &'static str = "PlotView";NAME2500,87301
    type ParentType = gtk::DrawingArea;ParentType2502,87345
    type Instance = PlotView;Instance2504,87386
    type Class = PlotViewClass;Class2505,87416
    fn class_init(klass: &mut PlotView) {class_init2509,87479
    fn new() -> Self {new2513,87575

/home/diego/Software/plots/src/context_mapper.rs,1544
pub struct Coord2D {Coord2D5,91
    pub fn new(x : f64, y : f64) -> Coord2D {new11,163
    pub fn distance(&self, other : Coord2D) -> f64 {distance15,238
    type Output=Self;Output23,413
    fn add(self, other: Self) -> Self {add25,436
pub struct ContextMapper {ContextMapper34,600
    fn default() -> Self {default51,902
    pub fn new(new78,1422
    pub fn update(&mut self) {update90,1816
    pub fn update_data_extensions(&mut self, xmin : f64, xmax : f64, ymin : f64, ymax : f64) {update_data_extensions96,2015
    pub fn update_dimensions(&mut self, w : i32, h : i32) {update_dimensions104,2244
    pub fn calc_ext(xmax : f64, xmin : f64, ymax : f64, ymin : f64,calc_ext110,2374
    pub fn set_mode(&mut self, xinv : bool, xlog : bool, yinv : bool, ylog : bool) {set_mode123,2802
    pub fn map(&self, x : f64, y : f64) -> Coord2D {map131,3021
    pub fn check_bounds(&self, x : f64, y : f64) -> bool {check_bounds159,4362
    pub fn coord_bounds(&self) -> (Coord2D, Coord2D, Coord2D, Coord2D) {coord_bounds173,4864
    pub fn data_extensions(&self) -> (f64, f64, f64, f64) {data_extensions182,5139
    pub fn coord_extensions(&self) -> (f64, f64) {coord_extensions186,5259
fn log_units(a : f64) -> f64 {log_units197,5635
fn round_to(b : f64, place : f64, up : bool) -> f64 {round_to203,5859
fn round_to_closest(val : f64, up : bool) -> f64 {round_to_closest213,6053
pub fn round_to_most_extreme(min : f64, max : f64) -> (f64, f64) {round_to_most_extreme225,6427
fn scales() {scales238,6753

/home/diego/Software/plots/src/mappings/scatter.rs,1599
pub struct ScatterMapping {ScatterMapping15,354
    fn default() -> Self {default26,541
    pub fn color(mut self, color : String) -> Self {color41,834
    pub fn radius(mut self, radius : f64) -> Self {radius46,952
    pub fn map<D>(x : impl IntoIterator<Item=D>, y : impl IntoIterator<Item=D>) -> Selfmap51,1054
    fn update(&mut self, prop : MappingProperty) -> bool {update80,1966
    fn clone_boxed(&self) -> Box<dyn Mapping> {clone_boxed95,2505
    fn update_from_json(&mut self, mut rep : super::super::json::Mapping) {update_from_json99,2591
    fn draw(&self, mapper : &ContextMapper, ctx : &Context) {draw113,3075
    fn update_data(&mut self, values : Vec<Vec<f64>>) {update_data135,3795
    fn update_extra_data(&mut self, _values : Vec<Vec<String>>) {update_extra_data140,3930
    fn properties(&self) -> HashMap<String, String> {properties166,4950
    fn mapping_type(&self) -> String {mapping_type186,5624
    fn get_col_name(&self, col : &str) -> String {get_col_name190,5695
    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))> {data_limits198,5906
    fn get_ordered_col_names(&self) -> Vec<(String,String)> {get_ordered_col_names206,6404
    fn get_hash_col_names(&self) -> HashMap<String, String> {get_hash_col_names213,6610
    fn set_col_name(&mut self, col : &str, name : &str) {set_col_name220,6851
    fn set_col_names(&mut self, cols : Vec<String>) -> Result<(), &'static str> {set_col_names228,7081
    fn set_source(&mut self, source : String) {set_source238,7381
    fn get_source(&self) -> String {get_source242,7466

/home/diego/Software/plots/src/mappings/bar.rs,1953
pub struct BarMapping {BarMapping16,560
    fn default() -> Self {default42,1306
    pub fn color(mut self, color : String) -> Self {color65,1870
    pub fn center_anchor(mut self, center_anchor : bool) -> Self {center_anchor71,2015
    pub fn width(mut self, w : f64) -> Self {width77,2173
    pub fn origin(mut self, origin : (f64, f64)) -> Self {origin83,2294
    pub fn bar_spacing(mut self, bar_spacing : f64) -> Self {bar_spacing89,2430
    pub fn horizontal(mut self, horizontal : bool) -> Self {horizontal95,2579
    pub fn map<D>(ext : impl IntoIterator<Item=D>) -> Selfmap101,2725
    fn adjust_bar(&mut self) {adjust_bar143,3972
    fn clone_boxed(&self) -> Box<dyn Mapping> {clone_boxed172,5073
    fn update(&mut self, prop : MappingProperty) -> bool {update176,5159
    fn update_from_json(&mut self, mut rep : super::super::json::Mapping) {update_from_json185,5372
    fn draw(&self, mapper : &ContextMapper, ctx : &Context) {draw190,5541
    fn update_data(&mut self, mut values : Vec<Vec<f64>>) {update_data224,7029
    fn update_extra_data(&mut self, _values : Vec<Vec<String>>) {update_extra_data239,7500
    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))> {data_limits298,10017
    fn properties(&self) -> HashMap<String, String> {properties306,10512
    fn mapping_type(&self) -> String {mapping_type347,12028
    fn get_col_name(&self, col : &str) -> String {get_col_name351,12095
    fn get_ordered_col_names(&self) -> Vec<(String, String)> {get_ordered_col_names361,12407
    fn get_hash_col_names(&self) -> HashMap<String, String> {get_hash_col_names370,12746
    fn set_col_name(&mut self, col : &str, name : &str) {set_col_name379,13116
    fn set_col_names(&mut self, cols : Vec<String>) -> Result<(), &'static str> {set_col_names389,13469
    fn set_source(&mut self, source : String) {set_source402,13925
    fn get_source(&self) -> String {get_source406,14010

/home/diego/Software/plots/src/mappings/surface.rs,1672
pub struct SurfaceMapping {SurfaceMapping14,335
    fn default() -> Self {default27,571
struct CoordPatch {CoordPatch44,1031
    pub fn map<D>(x : impl IntoIterator<Item=D>, y : impl IntoIterator<Item=D>, z : impl IntoIterator<Item=D>) -> Selfmap54,1183
    fn create_uniform_coords(mapper : &ContextMapper, n : usize) -> Vec<CoordPatch> {create_uniform_coords105,2659
    fn clone_boxed(&self) -> Box<dyn Mapping> {clone_boxed163,5064
    fn update(&mut self, prop : MappingProperty) -> bool {update167,5150
    fn update_from_json(&mut self, mut rep : super::super::json::Mapping) {update_from_json177,5352
    fn draw(&self, mapper : &ContextMapper, ctx : &Context) {draw185,5669
    fn update_data(&mut self, mut values : Vec<Vec<f64>>) {update_data264,8519
    fn update_extra_data(&mut self, _values : Vec<Vec<String>>) {update_extra_data311,10286
    fn properties(&self) -> HashMap<String, String> {properties355,12148
    fn mapping_type(&self) -> String {mapping_type387,13269
    fn get_col_name(&self, col : &str) -> String {get_col_name391,13340
    fn get_ordered_col_names(&self) -> Vec<(String,String)> {get_ordered_col_names400,13597
    fn get_hash_col_names(&self) -> HashMap<String, String> {get_hash_col_names408,13860
    fn set_col_name(&mut self, col : &str, name : &str) {set_col_name416,14161
    fn set_col_names(&mut self, cols : Vec<String>) -> Result<(), &'static str> {set_col_names425,14448
    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))> {data_limits436,14794
    fn set_source(&mut self, source : String) {set_source444,15289
    fn get_source(&self) -> String {get_source448,15374

/home/diego/Software/plots/src/mappings/mod.rs,2122
pub enum MappingType {MappingType29,452
    pub fn from_str(name : &str) -> Option<Self> {from_str41,576
    pub fn default_hash(&self) -> HashMap<String, String> {default_hash57,1231
fn update_data_pair_from_json(x : &mut Vec<f64>, y : &mut Vec<f64>, mut rep : super::json::Mapping) {update_data_pair_from_json104,3545
fn update_data_triplet_from_json(x : &mut Vec<f64>, y : &mut Vec<f64>, z : &mut Vec<f64>, mut rep : super::json::Mapping) {update_data_triplet_from_json115,3871
fn update_textual_data_from_json(x : &mut Vec<f64>, y : &mut Vec<f64>, z : &mut Vec<String>, mut rep : super::json::Mapping) {update_textual_data_from_json135,4462
pub fn new_from_json(mut rep : super::json::Mapping) -> Result<Box<dyn Mapping>, Box<dyn Error>> {new_from_json149,4909
pub trait MappingMapping192,6365
    fn draw(&self, mapper : &ContextMapper, ctx : &Context);// { }draw198,6439
    fn clone_boxed(&self) -> Box<dyn Mapping>;clone_boxed205,6804
    fn update(&mut self, prop : MappingProperty) -> bool;update207,6852
    fn update_data(&mut self, values : Vec<Vec<f64>>); //{ }update_data210,6941
    fn update_extra_data(&mut self, values : Vec<Vec<String>>);update_extra_data212,7003
    fn properties(&self) -> HashMap<String, String>;properties216,7141
    fn mapping_type(&self) -> String;mapping_type218,7195
    fn get_col_name(&self, col : &str) -> String;get_col_name220,7234
    fn get_ordered_col_names(&self) -> Vec<(String, String)>;get_ordered_col_names222,7285
    fn get_hash_col_names(&self) -> HashMap<String, String>;get_hash_col_names224,7348
    fn set_col_name(&mut self, col : &str, name : &str);set_col_name226,7410
    fn set_col_names(&mut self, cols : Vec<String>) -> Result<(), &'static str>;set_col_names228,7468
    fn set_source(&mut self, source : String);set_source230,7550
    fn get_source(&self) -> String;get_source232,7598
    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))>;data_limits234,7635
    fn update_from_json(&mut self, rep : super::json::Mapping);update_from_json236,7699
    fn clone(&self) -> Self {clone242,7802

/home/diego/Software/plots/src/mappings/line.rs,1719
pub struct LineMapping {LineMapping19,504
    fn default() -> Self {default31,702
    pub fn width(mut self, w : f64) -> Self {width47,1015
    pub fn color(mut self, color : String) -> Self {color52,1105
    pub fn dash_n(mut self, dash_n : i32) -> Self {dash_n57,1223
    pub fn map<D>(x : impl IntoIterator<Item=D>, y : impl IntoIterator<Item=D>) -> Selfmap63,1353
    fn build_dash(n : i32) -> Vec<f64> {build_dash95,2341
    fn update(&mut self, prop : MappingProperty) -> bool {update108,2593
    fn clone_boxed(&self) -> Box<dyn Mapping> {clone_boxed124,3175
    fn draw(&self, mapper : &ContextMapper, ctx : &Context) {draw128,3261
    fn update_data(&mut self, values : Vec<Vec<f64>>) {update_data171,4729
    fn update_from_json(&mut self, mut rep : super::super::json::Mapping) {update_from_json176,4864
    fn update_extra_data(&mut self, _values : Vec<Vec<String>>) {update_extra_data193,5415
    fn properties(&self) -> HashMap<String, String> {properties223,6617
    fn mapping_type(&self) -> String {mapping_type246,7391
    fn get_col_name(&self, col : &str) -> String {get_col_name250,7459
    fn get_ordered_col_names(&self) -> Vec<(String, String)> {get_ordered_col_names258,7670
    fn get_hash_col_names(&self) -> HashMap<String, String> {get_hash_col_names265,7877
    fn set_col_name(&mut self, col : &str, name : &str) {set_col_name272,8118
    fn set_col_names(&mut self, cols : Vec<String>) -> Result<(), &'static str> {set_col_names280,8348
    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))> {data_limits290,8648
    fn set_source(&mut self, source : String) {set_source298,9143
    fn get_source(&self) -> String {get_source302,9228

/home/diego/Software/plots/src/mappings/area.rs,1614
pub struct AreaMapping {AreaMapping18,477
    fn default() -> Self {default29,664
    pub fn color(mut self, color : String) -> Self {color44,977
    pub fn map<D>(x : impl IntoIterator<Item=D>, ymin : impl IntoIterator<Item=D>, ymax : impl IntoIterator<Item=D>) -> Selfmap49,1095
    pub fn draw_bound<'a>(draw_bound82,2235
    fn update(&mut self, prop : MappingProperty) -> bool {update113,3097
    fn clone_boxed(&self) -> Box<dyn Mapping> {clone_boxed126,3367
    fn update_from_json(&mut self, mut rep : super::super::json::Mapping) {update_from_json130,3453
    fn draw(&self, mapper : &ContextMapper, ctx : &Context) {draw140,3763
    fn update_data(&mut self, values : Vec<Vec<f64>>) {update_data181,5432
    fn properties(&self) -> HashMap<String, String> {properties212,6636
    fn mapping_type(&self) -> String {mapping_type235,7421
    fn get_col_name(&self, col : &str) -> String {get_col_name239,7489
    fn get_ordered_col_names(&self) -> Vec<(String, String)> {get_ordered_col_names248,7749
    fn get_hash_col_names(&self) -> HashMap<String, String> {get_hash_col_names256,8018
    fn set_col_name(&mut self, col : &str, name : &str) {set_col_name264,8322
    fn update_extra_data(&mut self, _values : Vec<Vec<String>>) {update_extra_data273,8612
    fn set_col_names(&mut self, cols : Vec<String>) -> Result<(), &'static str> {set_col_names277,8735
    fn set_source(&mut self, source : String) {set_source288,9084
    fn get_source(&self) -> String {get_source292,9169
    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))> {data_limits296,9241

/home/diego/Software/plots/src/mappings/text.rs,1712
pub struct TextMapping {TextMapping18,467
    fn default() -> Self {default30,675
    pub fn color(mut self, color : String) -> Self {color46,1024
    pub fn font(mut self, font : String) -> Self {font51,1142
    pub fn map<D, T>(x : impl IntoIterator<Item=D>, y : impl IntoIterator<Item=D>, text : impl IntoIterator<Item=T>) -> Selfmap56,1267
    pub fn set_text_data(&mut self, text : &Vec<String>) {set_text_data95,2630
    fn clone_boxed(&self) -> Box<dyn Mapping> {clone_boxed102,2764
    fn update(&mut self, prop : MappingProperty) -> bool {update106,2850
    fn update_from_json(&mut self, mut rep : super::super::json::Mapping) {update_from_json122,3441
    fn draw(&self, mapper : &ContextMapper, ctx : &Context) {draw135,3887
    fn update_data(&mut self, values : Vec<Vec<f64>>) {update_data167,4986
    fn update_extra_data(&mut self, values : Vec<Vec<String>>) {update_extra_data172,5121
    fn properties(&self) -> HashMap<String, String> {properties202,6244
    fn mapping_type(&self) -> String {mapping_type225,7021
    fn get_col_name(&self, col : &str) -> String {get_col_name229,7089
    fn get_ordered_col_names(&self) -> Vec<(String,String)> {get_ordered_col_names238,7349
    fn set_col_name(&mut self, col : &str, name : &str) {set_col_name246,7618
    fn get_hash_col_names(&self) -> HashMap<String, String> {get_hash_col_names255,7908
    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))> {data_limits267,8531
    fn set_col_names(&mut self, cols : Vec<String>) -> Result<(), &'static str> {set_col_names278,9137
    fn set_source(&mut self, source : String) {set_source289,9486
    fn get_source(&self) -> String {get_source293,9571

/home/diego/Software/plots/src/mappings/interval.rs,1918
pub struct IntervalMapping {IntervalMapping15,356
    fn default() -> Self {default30,625
    pub fn limit_size(mut self, sz : f64) -> Self {limit_size49,1046
    pub fn vertical(mut self, vertical : bool) -> Self {vertical54,1144
    pub fn width(mut self, w : f64) -> Self {width59,1255
    pub fn color(mut self, color : String) -> Self {color64,1345
    pub fn dash_n(mut self, dash_n : i32) -> Self {dash_n69,1463
    pub fn map<D>(x : impl IntoIterator<Item=D>, ymin : impl IntoIterator<Item=D>, ymax : impl IntoIterator<Item=D>) -> Selfmap74,1565
    fn build_dash(n : i32) -> Vec<f64> {build_dash111,2848
    fn update(&mut self, prop : MappingProperty) -> bool {update124,3104
    fn clone_boxed(&self) -> Box<dyn Mapping> {clone_boxed143,3939
    fn draw(&self, mapper : &ContextMapper, ctx : &Context) {draw147,4025
    fn update_data(&mut self, values : Vec<Vec<f64>>) {update_data211,6916
    fn update_from_json(&mut self, mut rep : super::super::json::Mapping) {update_from_json217,7093
    fn update_extra_data(&mut self, _values : Vec<Vec<String>>) {update_extra_data240,7850
    fn properties(&self) -> HashMap<String, String> {properties270,9052
    fn mapping_type(&self) -> String {mapping_type296,9937
    fn get_col_name(&self, col : &str) -> String {get_col_name300,10009
    fn get_ordered_col_names(&self) -> Vec<(String, String)> {get_ordered_col_names309,10272
    fn get_hash_col_names(&self) -> HashMap<String, String> {get_hash_col_names318,10576
    fn set_col_name(&mut self, col : &str, name : &str) {set_col_name326,10846
    fn set_col_names(&mut self, cols : Vec<String>) -> Result<(), &'static str> {set_col_names335,11105
    fn data_limits(&self) -> Option<((f64, f64), (f64, f64))> {data_limits346,11434
    fn set_source(&mut self, source : String) {set_source362,12531
    fn get_source(&self) -> String {get_source366,12616

/home/diego/Software/plots/src/scale.rs,1429
pub enum Adjustment {Adjustment13,577
    type Err = ();Err23,665
    fn from_str(s : &str) -> Result<Self, ()> {from_str25,685
    fn default() -> Self {default38,943
pub struct Scale {Scale45,1022
    fn default() -> Self {default60,1300
    pub fn label(mut self, label : &str) -> Self {label80,1703
    pub fn precision(mut self, precision : i32) -> Self {precision85,1814
    pub fn extension(mut self, from : f64, to : f64) -> Self {extension90,1928
    pub fn intervals(mut self, n : i32) -> Self {intervals97,2088
    pub fn log(mut self, log : bool) -> Self {log103,2217
    pub fn invert(mut self, invert : bool) -> Self {invert109,2337
    pub fn offset(mut self, offset : i32) -> Self {offset115,2469
    pub fn adjustment(mut self, adj : Adjustment) -> Self {adjustment121,2600
    pub fn new() -> Self {new127,2733
    pub fn update(&mut self, prop : ScaleProperty) {update131,2824
    fn update_steps(&mut self) {update_steps168,3905
    pub fn new_from_json(rep : super::json::Scale) -> Option<Self> {new_from_json172,4041
    pub fn new_full(new_full181,4396
    pub fn description(&self) -> HashMap<String, String> {description196,4805
pub fn adjust_segment(seg : &mut Scale, adj : Adjustment, data_min : f64, data_max : f64) {adjust_segment217,5639
fn define_steps(n_intervals : i32, from : f64, to : f64, offset : i32, log : bool) -> Vec<f64> {define_steps239,6526

/home/diego/Software/plots/src/text.rs,467
pub struct FontData {FontData10,183
    fn default() -> Self {default20,382
    pub fn create_standard_font() -> Self {create_standard_font39,835
    pub fn new_from_string(font : &str) -> Self {new_from_string53,1280
    pub fn description(&self) -> String {description99,2896
    pub fn set_font_into_context(&self, ctx : &Context) {set_font_into_context116,3423
fn create_scaled_font(create_scaled_font126,3670
pub fn draw_label(draw_label150,4398

/home/diego/Software/plots/src/plot_design.rs,270
pub struct PlotDesign {PlotDesign11,210
    fn default() -> Self {default20,376
    pub fn new_from_json(rep : super::json::Design) -> Result<Self, Box<dyn Error>> {new_from_json33,674
    pub fn description(&self) -> HashMap<String, String> {description65,1864
/home/diego/.rusty-tags/cache/base64-14376625922731224138.emacs,include
/home/diego/.rusty-tags/cache/cairo-rs-10603161284976133194.emacs,include
/home/diego/.rusty-tags/cache/either-4758121256392261169.emacs,include
/home/diego/.rusty-tags/cache/gdk-pixbuf-8819896343374692236.emacs,include
/home/diego/.rusty-tags/cache/gdk4-3657412171092641097.emacs,include
/home/diego/.rusty-tags/cache/regex-12998750978332741002.emacs,include
/home/diego/.rusty-tags/cache/serde-14889227222400872623.emacs,include
/home/diego/.rusty-tags/cache/serde_json-12714474309321956859.emacs,include
/home/diego/.rusty-tags/cache/showable-12239105926454063289.emacs,include
/home/diego/.rusty-tags/cache/tempfile-8747781834504598941.emacs,include
