use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

//old performance number: 650 ms
fn main() {
    let small_cavern = Cavern::<10>::from_str(PRACTICE);
    small_cavern.print();
    println!("Min path for small cavern: {}\n", small_cavern.get_cost());

    let large_cavern = Cavern::<100>::from_str(INPUT);
    large_cavern.print();
    println!("Min path for large cavern: {}\n", large_cavern.get_cost());

    let medium_cavern = Cavern::<50>::larger_from_str(PRACTICE, 10);
    medium_cavern.print();
    println!("Min path for medium cavern: {}\n", medium_cavern.get_cost());

    let start = Instant::now();
    let huge_cavern = Cavern::<500>::larger_from_str(INPUT, 100);
    let time = start.elapsed();
    //huge_cavern.print();
    println!("{time:#?}");
    println!("Min path for huge cavern: {}\n", huge_cavern.get_cost());
}

#[derive(Debug)]
struct Cavern<const SIZE: usize> {
    risk_levels: [[u8; SIZE]; SIZE],
    graph: HashMap<(usize, usize), Vec<GraphEdge>>,
    searched: HashMap<(usize, usize), SearchNode>,
    searching: HashMap<(usize, usize), SearchNode>,
}

#[derive(Debug)]
struct GraphEdge {
    key: (usize, usize),
    cost: u8,
}

#[derive(Debug, Clone, Copy)]
struct SearchNode {
    cost: usize,
    parent: Option<(usize, usize)>,
}

impl<const SIZE: usize> Cavern<SIZE> {
    fn get_cost(&self) -> usize {
        self.searched.get(&(SIZE - 1, SIZE - 1)).unwrap().cost
    }

    fn search(&mut self) {
        while let Some((key, node)) = self
            .searching
            .iter()
            //copy values instead of returning references so hashmap can be updated later
            .map(|(k, v)| (*k, *v))
            .min_by_key(|(_, v)| v.cost)
        {
            let edges = self.graph.get(&key).unwrap();
            for edge in edges {
                if !self.searched.contains_key(&edge.key) {
                    let calculated_cost = node.cost + edge.cost as usize;
                    match self.searching.entry(edge.key) {
                        std::collections::hash_map::Entry::Occupied(mut occ) => {
                            let old_neighbor = occ.get_mut();
                            if calculated_cost < old_neighbor.cost {
                                *old_neighbor = SearchNode {
                                    cost: calculated_cost,
                                    parent: Some(key),
                                }
                            }
                        }
                        std::collections::hash_map::Entry::Vacant(vac) => {
                            vac.insert(SearchNode {
                                cost: calculated_cost,
                                parent: Some(key),
                            });
                        }
                    }
                }
            }
            self.searching.remove(&key);
            self.searched.insert(key, node);
        }
    }

    fn larger_from_str(s: &str, size: usize) -> Self {
        let mut risk_levels = [[0u8; SIZE]; SIZE];
        let mut inputs = s
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()));
        for row in 0..size {
            for col in 0..size {
                risk_levels[row][col] = inputs.next().unwrap() as u8;
            }
        }

        for row in 0..SIZE {
            for col in 0..SIZE {
                risk_levels[row][col] =
                    ((((risk_levels[row % size][col % size] as usize + row / size + col / size)
                        - 1)
                        % 9)
                        + 1) as u8;
            }
        }

        Self::make_cavern(risk_levels)
    }
    fn from_str(s: &str) -> Self {
        let mut risk_levels = [[0u8; SIZE]; SIZE];
        risk_levels
            .iter_mut()
            .flatten()
            .zip(
                s.lines()
                    .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap())),
            )
            .for_each(|(i, j)| *i = j as u8);

        Self::make_cavern(risk_levels)
    }
    fn make_cavern(risk_levels: [[u8; SIZE]; SIZE]) -> Self {
        let mut graph = HashMap::new();
        for row in 0..SIZE {
            for col in 0..SIZE {
                let mut edges = vec![];
                if row > 0 {
                    edges.push(GraphEdge {
                        key: (row - 1, col),
                        cost: risk_levels[row - 1][col],
                    })
                }
                if col > 0 {
                    edges.push(GraphEdge {
                        key: (row, col - 1),
                        cost: risk_levels[row][col - 1],
                    })
                }
                if row < SIZE - 1 {
                    edges.push(GraphEdge {
                        key: (row + 1, col),
                        cost: risk_levels[row + 1][col],
                    })
                }
                if col < SIZE - 1 {
                    edges.push(GraphEdge {
                        key: (row, col + 1),
                        cost: risk_levels[row][col + 1],
                    })
                }
                graph.insert((row, col), edges);
            }
        }
        let mut searching = HashMap::new();
        searching.insert(
            (0, 0),
            SearchNode {
                parent: None,
                cost: 0,
            },
        );

        let mut out = Self {
            risk_levels,
            graph,
            searching,
            searched: HashMap::new(),
        };
        out.search();
        out
    }
    fn print(&self) {
        use std::{collections::HashSet, io::Write};
        use termcolor::{BufferedStandardStream, Color, ColorChoice, ColorSpec, WriteColor};
        let mut set = HashSet::new();
        set.insert((SIZE - 1, SIZE - 1));
        let mut end = (SIZE - 1, SIZE - 1);
        while let Some(parent) = self.searched.get(&end).unwrap().parent {
            set.insert(parent);
            end = parent;
        }
        let mut stdout = BufferedStandardStream::stdout(ColorChoice::Always);
        let mut green = ColorSpec::new();
        green.set_fg(Some(Color::Green)).set_bold(true);
        let default = ColorSpec::default();
        for row in 0..SIZE {
            for col in 0..SIZE {
                match set.contains(&(row, col)) {
                    false => {
                        write!(&mut stdout, "{}", self.risk_levels[row][col]).unwrap();
                    }
                    true => {
                        stdout.set_color(&green).unwrap();
                        write!(&mut stdout, "{}", self.risk_levels[row][col]).unwrap();
                        stdout.set_color(&default).unwrap();
                    }
                }
            }
            write!(&mut stdout, "\n").unwrap();
        }
    }
}

const PRACTICE: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

const INPUT: &str = "2494188795949368426672935997553436999123973817111494977966298896479213563868828787114774871959993495
7269295166912819128199218896192745233371189151296812199288995976199874739992471696989914173889976925
5191212388827998986649912824634749279971948589689238817721846893532697679994545789897911889729995897
2168589418228699841979496964487152857627622699771987389157912193827151789199146397936892984945186648
6173939628157711976563997464999974959831921613389971361747448844585131388314689679172394799717331399
5899695651247379821996866399919649874365217617874746871338598594867719518499554272189235565993897965
7485394112785944858863361332818786294335182341788296167915326872949943697179245577456182671381554981
9266892719919136231211827869936952321796769156938997799961743957799879299829979797999289996519698258
7917921979835316979873649974359699969481697221634859899429669995158723957697928969198838877882679431
1491236284143364659399923722198867919166499918548183212698729886479216469986191478946997662853172832
3767398586846967844992213195183588959313371637243199793963993959286917791341794214391371939868871494
9928742956264979397583828698198991474967144891939974212795669284197972936548635221944579878942359557
2591296163233973188949425669798998818121787829476539915785989899996476991741928456884321481461491491
9991453999117657667486988919926741998881973198189217569482699982656854861887538683949945599464996179
7785121782479268191396996844959721953747843271399498791933669879859637175866291876796575659279993189
7776716891792899824695392599338137499166496894838139256577699799459229235864327718998999599938889161
5839791979939975731327544972918993669998884665789489989759759159981911739962619655578959489319651959
9398937827918996618898187917114312218979818328154357119792473517498229889994917199791479298969989426
9193149311416478995456723118877968917584864848296657785894974918318495982911493994794738911468372399
9769827996996299982985113892343422898578915399986948881116926857688725599882867388823188133721994278
8818187692494197511189732662739876774788519148132878987446213196194758987589658244842573992965369917
9692985985717419377379789693788389987764679742169692881958792669418357791948997197357944583299997199
1655589698936188425679138917313955276139596613919192843881592947255229449287767987981926864969859529
9699242895916761696979961626847898651779897127819171988159298969595969856231117848529246839289723796
6918899858935889816997979865999596479948574186514639447389189297279388214912422481934482758598686939
9996124999974921628385339724671481964691768899569149945536398488739568172834846547989816384911492872
8346834917676292979994979698956139979926678894814989654839981221562573298658346721681719173725669956
7996658985879211759466921616176161867986927757827999273979469498688699982778999718998879599891187826
6789987138357992752226589295829589178911828185979998763692162929329395765591169737899686919889788875
7197637335578924955696669176672558865939999937795981568398933385959211553997726292683778819899973695
7969119547891678248796699588889994968987939996198215675832711397499256187159534154919899797617918187
6926911899812131895651119837176697797919392241689685965558165297919473987229989972939921478114978373
5841994288699114592993125898675872864168987969699196889625973399229985914231352968138691482158881899
6867891298189866894719212682879398889498489939663172817599835471792399556867597964989686389769859994
9216183839556753967949862931537817787984781697457685114963135184139788699929447794287121297683798319
5961184475118971259559817821178849966985528319614894877141814318889588689738389571282791915658998542
7796517463682296191129934366199787898432878578178838999284446321338627649991196629939797121785891962
6869179819567982969986839939999929197979374876888941369191219962943812332887931152176916191949337191
9949555298548218159821548895199792894846453957998718924987391181922787245277994179172391848939291937
9194554898149494418187494726757793324999959688688219769989897645955852991449724578587938397525559699
9839395266191998228728197617917693697789375996118295993584569962144786399964589627981549488787733896
8798892947496976899997799782965947775618684631612298893931561678929111719994767999284582998895629195
5971923418391999545117998893871871449928897518896798992962997798871699831697768939889162189217948177
5858919812839818645171731976975688955947178448898559483839791819932171192959869296448582448911965279
1968371935371579114969391889167369857978852993227571182176195567113584591687978173852172998299297876
9818595991179878228913865926959618915114684669669687682418328328426959971759423266149164699675299962
7313169988896798377992637828717558819632818939589195963274459828798159996969414246778645986912959685
7175987582985899179876194819674578385889199196894972994931968897971126896672969822346958482487693195
3899899994999974339297695494999694587728925279418719657933319839559165236843874998959197285872898386
9143179894278789613961188998997899879989984167462521348529727699979885411983992819969799931648899396
8721719519784582186657994994597695787791988737117281949127388845793462485588977564523597916812662293
8611192812559976315185196299139868529583964194698936966669193992848592369935883977685998128921958855
9939897898864487468688938157636793892493993149941579971537776639995925839997995494499799341196996911
8952975169934893855698711783493617877741259797391888882838674537287761499137286977596911311795914798
5973819164498977729977283879968689716999899829329989552936969439956928519569699593711189175982816879
8992169969711179987999697359795943869189334168512958869138486551393962896928198678797618959592667178
5198893235999189515823758534932887297573118289138717189899951151794645128875863218397267732544682559
1325921986621997358689228699823234199583753862125884477531428896914475773981149951219987772989777284
1767578881144877899227864881927651792585225919996798399996398994171539489225941719236558689987289596
1697693293932998829129311328298516993977484911198964347455679799762539363996979349779755395869958127
3713898193499495896831949473519979859286519968829254986836974926445967798179318189714751879952896613
9916216881218784598988194111975928978834317778788697936768819246855919484199948946983872997799989537
5418692222281246379686836239999941778791771954439968621669899592193586891966947915978832988949879767
5946329868375678638572651999898943139289184131912762798677877895898695944967941764178789929296788749
6239775996391914149474397187426592989328499959996744188988177257137398164298656927588639947169611996
7726974769124948598778698999782543243598997487367127713261541411788169989779736617197163837998429443
6111148329281896798281927928447986973659988162159399599953814156215976289871288499437886599369129446
5854951519241889185939295929639732174661849974497795351828153777386926499257888278938847986849719938
5984221699399349924974897835513966246932291529259259986518919741383138819187699681468921932862191135
7247744517283916799878894898841511684972958968888598899899528129686787319513181999663332357287917849
9619575771852888429993998185894999449977654581892868719896632889994183982173959417945797894859792989
9987887311965149698616929977895196273977999972615874679953263114467325584669891293619617955392613925
3459233982935649168889415851457199862587819992969777358114793166474749199299992637999874364191376899
7943688928864588361988811191325483948768896478821434855799291838959158636199538793219669994353374759
7859289672994646496294159151719934379268389791695698878689997723696963668819627473895291179249595592
9941314869878731534751835338688822211841761128799388619893279239832929889693971299176198322989988696
8842227781495829982998213979935669336595995299977597191369528699162399363739696583936699198339998297
4713836572615268829578998379651451979528877965188284221548263244343828575977462755943999496539416928
8997969648694117619993817848699798391349937895138998191853188328148899863968896951342872529854299799
4821998179179181915669967943833899175819993535997526988947429178781551978844594192169998492291957896
4195189588992241478996816697172978669351926159581981999991898899275537372449313988179685677999419998
7562897828594989799593999721399698168294159217347891591919627938573198351476731278382919975715726691
5289289886391592468511382946949159281295428969892859956998441988997291849472619345291529291989934296
2119968215975689967875154978999999387426829368331479442781739197214991219186498848963987197926389319
2619398714969892292337865366952999489167916889966465489596798298768869549286862389939419119395283486
5891141991985484321796885478397617796913878991923119711484499287684339488268919291995478738911979334
8114983942818278171367398195885616828459999126819995888739198519259919986496918279919325994993862348
9799778199299926255679191711375949221688768118898797284688592829217988489896761335847548468981782899
3297828241798981838196692979721897463899191684523229918888914388941777196891967689798993724124799799
6587642855381998719928875658181669971111961763447368283978979892939694984843414921587564196767985264
8868499779173993666956935631879522347992698928849354893756692376863389161268189398695789183996886289
4384313399189397465391981199547955659912476999789237991829967965898994592593315769962549121389114869
3165989928816884816238178158923873189899685775749877366272667891986999279928649289915951524688386419
2614193692887593891959411118711929833726848873925563723329796828281367982397811129881655482798978846
8547195678964249376891881397319282899129476666886297539869668898819828797769638991899886419148826859
8178778491299194939588873989868848883795996377199995921984895879486959879816117891527867619711458789
2358691498849138995117937966497995947487954198989213589121699888997868911988859219584998799742529591
4564249477614979424868747948799188548777892792598985751974975274769994898988419988258297291881465592
9297979945165922849989776118448916916471698458557921916479112985169284661994778819637195998913985273
4789996397188799369699683969576838983199839289799579353875318987692211979981993139884198857448759796";
