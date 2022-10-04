use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};
const MATCH_THRESHOLD: u32 = 12;

fn main() {
    let scanners = PRACTICE
        .split("\n\n")
        .map(|block| from_str(block))
        .collect();
    let (located, max_dist) = locate_beacons(scanners);

    println!(
        "total beacons: {}, largest distance: {max_dist}",
        count_beacons(&located)
    );

    let start = Instant::now();
    let scanners = INPUT.split("\n\n").map(|block| from_str(block)).collect();
    let (located, max_distance) = locate_beacons(scanners);
    let total_beacons = count_beacons(&located);
    let time = start.elapsed();

    println!("{total_beacons} beacons and a distance of {max_distance} between scanners found in {time:#?}");
}

fn count_beacons(beacons: &Vec<Vec<Beacon>>) -> usize {
    let set: HashSet<Beacon> = beacons.iter().flatten().map(|val| val.clone()).collect();
    set.len()
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Diff {
    x: i32,
    y: i32,
    z: i32,
}

fn from_str(val: &str) -> Vec<Beacon> {
    let mut set = vec![];
    for line in val.lines().skip(1) {
        let mut nums = line.split(',');
        set.push(Beacon {
            x: nums.next().unwrap().parse().unwrap(),
            y: nums.next().unwrap().parse().unwrap(),
            z: nums.next().unwrap().parse().unwrap(),
        });
    }
    set
}

fn locate_beacons(mut scanners: Vec<Vec<Beacon>>) -> (Vec<Vec<Beacon>>, i32) {
    let mut located = vec![scanners.pop().unwrap()];
    let mut finished = vec![];
    let mut scanner_positions = vec![];
    while located.len() > 0 {
        let mut newly_located = vec![];
        for loc in located {
            let mut unfound_scanners = vec![];
            for mut scan in scanners {
                match locate(&loc, &mut scan) {
                    Some(diff) => {
                        newly_located.push(scan);
                        scanner_positions.push(diff);
                        //println!("located a scanner!")
                    }
                    None => unfound_scanners.push(scan),
                }
            }
            scanners = unfound_scanners;
            finished.push(loc);
        }
        located = newly_located;
    }
    let mut max = 0;
    for outer in &scanner_positions {
        for inner in &scanner_positions {
            let distance =
                (outer.x - inner.x).abs() + (outer.y - inner.y).abs() + (outer.z - inner.z).abs();
            max = std::cmp::max(max, distance);
        }
    }

    (finished, max)
}

fn locate(grounded: &Vec<Beacon>, lost: &mut Vec<Beacon>) -> Option<Diff> {
    for _ in 0..2 {
        for _ in 0..3 {
            if let Some(diff) = globalize_points(grounded, lost) {
                return Some(diff);
            }
            for _ in 0..3 {
                rot_z(lost);
                if let Some(diff) = globalize_points(grounded, lost) {
                    return Some(diff);
                }
            }
            rot_y(lost);
        }
        rot_z(lost);
        rot_y(lost);
        rot_y(lost);
    }

    None
}

fn globalize_points(global_points: &Vec<Beacon>, local_points: &mut Vec<Beacon>) -> Option<Diff> {
    let mut diffs = HashMap::<Diff, u32>::new();
    let mut out = None;
    'outer: for glob in global_points {
        for loc in local_points.iter() {
            match diffs.entry(subtract(glob, loc)) {
                std::collections::hash_map::Entry::Occupied(mut occ) => {
                    let val = occ.get_mut();
                    *val += 1;
                    if *val >= MATCH_THRESHOLD {
                        out = Some(subtract(glob, loc));
                        break 'outer;
                    }
                }
                std::collections::hash_map::Entry::Vacant(vac) => {
                    vac.insert(1);
                }
            }
        }
    }
    match out {
        Some(difference) => {
            for point in local_points.iter_mut() {
                point.x += difference.x;
                point.y += difference.y;
                point.z += difference.z;
            }
            Some(difference)
        }
        None => {
            //println!("{}, {}", diffs.values().max().unwrap(), diffs.len());
            None
        }
    }
}
fn rot_z(scanner: &mut Vec<Beacon>) {
    for beacon in scanner {
        let temp = beacon.y;
        beacon.y = beacon.x;
        beacon.x = -temp;
    }
}
fn rot_y(scanner: &mut Vec<Beacon>) {
    for beacon in scanner {
        let temp = beacon.z;
        beacon.z = beacon.x;
        beacon.x = -temp;
    }
}

fn subtract(lhs: &Beacon, rhs: &Beacon) -> Diff {
    Diff {
        x: lhs.x - rhs.x,
        y: lhs.y - rhs.y,
        z: lhs.z - rhs.z,
    }
}

const PRACTICE: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

const INPUT: &str = "--- scanner 0 ---
518,-324,325
-757,492,-559
-804,-678,847
-64,-12,64
371,698,-494
467,-524,299
-920,-609,814
428,613,-347
731,378,348
467,-704,-403
-766,564,-509
-581,-437,-419
785,563,361
-732,-659,761
-157,-74,-106
282,582,-425
387,-696,-454
-570,-484,-507
-654,350,668
-760,359,665
-522,-424,-636
499,-332,281
693,551,425
333,-713,-333
-740,458,696
-770,423,-443

--- scanner 1 ---
371,568,-587
-599,701,-690
-513,-505,867
730,-849,-571
-445,709,906
568,323,494
589,363,707
-544,-470,921
-555,768,923
527,445,588
-158,-85,60
-411,-823,-426
309,464,-682
-5,43,125
257,-498,922
661,-760,-455
241,-685,891
-447,-900,-351
-599,742,-625
-496,-440,728
-551,-849,-451
808,-703,-540
432,626,-671
255,-669,958
-621,725,-667
-410,710,843

--- scanner 2 ---
-701,-443,-920
554,-925,-548
-505,704,-530
419,431,418
-513,-502,-879
-109,-145,-127
470,-909,-731
-564,-674,438
-752,371,454
-700,393,567
666,-876,-680
-671,688,-596
417,-684,686
506,-629,736
390,671,-790
-624,-644,341
-162,-69,52
469,345,375
324,-667,704
463,776,-910
-698,-643,418
-577,-474,-904
486,265,414
-522,788,-569
342,786,-839
-714,526,517

--- scanner 3 ---
745,-414,606
98,32,-1
584,-745,-450
-459,807,528
674,891,-539
396,761,746
643,-643,-519
-556,698,506
502,882,-596
-583,-400,620
-510,-464,752
759,-432,575
-564,-449,655
-615,759,549
-344,907,-491
671,-820,-556
-466,-577,-463
-545,-585,-529
-467,933,-414
817,-391,573
567,935,-641
-462,800,-552
-474,-765,-513
421,825,718
380,637,777

--- scanner 4 ---
418,688,451
-688,-444,530
735,-462,-655
-447,822,753
618,721,466
-801,300,-569
-466,736,681
-516,789,869
852,733,-797
873,528,-797
514,568,466
594,-768,706
-793,256,-550
850,550,-772
600,-812,631
472,-833,596
-701,-661,471
107,-143,36
-421,-428,-518
-601,274,-518
-641,-532,519
-429,-479,-438
535,-482,-587
-412,-358,-384
621,-410,-695

--- scanner 5 ---
-108,-59,-141
627,313,281
-592,-712,-605
-487,683,553
-752,428,-384
-666,-346,805
-450,-594,-573
814,-564,591
643,380,267
836,-381,588
430,-700,-550
59,-32,-32
-87,67,39
-751,365,-419
-713,-385,821
-654,-420,633
-571,-500,-647
782,-483,530
456,349,283
-405,564,615
-810,348,-529
426,-544,-637
-365,777,594
422,-539,-462
463,404,-487
398,543,-470
428,350,-518

--- scanner 6 ---
743,320,649
-758,-625,753
796,304,-726
710,-783,-638
516,-869,482
-915,595,-514
568,422,671
414,-953,506
681,353,-678
593,377,728
-723,607,919
674,-843,-435
436,-961,451
-486,-430,-529
-553,645,861
-867,-502,806
-866,463,-402
-465,-399,-496
-899,412,-598
-645,714,844
786,233,-605
740,-960,-549
-693,-560,751
-675,-437,-542
21,-94,89

--- scanner 7 ---
494,879,534
438,836,579
548,-792,-893
-853,669,682
-415,571,-714
545,-592,445
360,-733,-852
-425,665,-760
-819,-359,-653
32,64,-17
-707,-310,-521
-888,632,562
411,-561,465
-533,540,-709
471,-591,368
-570,-551,713
-659,-606,763
424,-761,-877
303,581,-771
-623,-326,-704
294,524,-599
250,626,-706
-135,-69,14
341,833,565
-477,-568,731
-760,706,566

--- scanner 8 ---
795,755,647
277,-364,-462
-864,-338,-607
-942,-484,-576
417,-338,-487
502,509,-840
-650,486,578
-662,527,450
-391,-689,886
27,-32,-62
-712,608,542
410,-717,885
-389,-518,922
362,-747,692
409,436,-792
379,-682,831
586,394,-774
-780,710,-625
-730,701,-697
313,-287,-387
-375,-502,855
822,730,622
-159,13,21
730,713,775
-795,-427,-491
-846,714,-766

--- scanner 9 ---
547,-387,715
634,495,479
563,-496,-567
-637,-417,-717
-437,925,-704
408,-433,-529
-840,-705,693
-607,-320,-657
741,551,-776
-610,504,448
560,434,446
-577,576,471
-876,-681,614
430,-546,-581
-788,-702,470
-406,941,-580
618,-345,803
825,590,-759
-608,562,489
-109,-47,-74
-465,900,-768
650,404,359
-25,34,68
-606,-397,-583
725,617,-610
762,-326,738

--- scanner 10 ---
-822,-854,486
-503,681,509
592,-666,-332
-678,781,-366
-59,-27,58
447,-408,479
2,-131,-84
452,325,-407
582,405,-479
579,-696,-350
-830,-926,-623
-791,-818,361
512,438,683
623,-680,-345
-698,-738,474
-748,-874,-550
-729,807,-566
379,476,693
-697,825,-374
441,-508,355
585,329,-522
-854,-753,-556
435,471,821
-411,745,521
461,-514,425
-465,758,336

--- scanner 11 ---
-490,801,756
-770,-350,518
-822,-407,659
-513,758,631
682,477,652
-731,729,-763
-862,-529,-479
378,-671,556
717,526,-657
699,557,-671
-7,44,90
-805,-693,-429
-753,-622,-557
-452,811,721
-766,-501,599
740,559,-729
709,-685,-738
731,512,671
667,380,598
-88,-38,-71
326,-641,513
378,-725,522
-824,789,-706
-846,781,-723
742,-611,-799
829,-701,-829

--- scanner 12 ---
130,21,-111
-656,-401,-533
-702,759,697
554,-554,-934
-585,-259,-532
519,732,677
446,-653,-904
-16,170,-52
735,802,-897
531,-604,-952
-753,-281,783
-798,-450,757
760,942,-871
-591,-302,-583
559,-582,732
496,754,653
561,754,694
-674,789,603
-636,-416,810
835,906,-948
-780,544,-489
-681,532,-515
521,-555,607
-705,691,696
522,-683,577
-727,449,-443

--- scanner 13 ---
-387,408,298
-360,-677,-365
819,400,647
-794,-575,446
-328,596,257
891,-568,267
-359,472,273
803,566,581
-577,426,-582
134,-81,19
-570,-631,-403
-761,-771,497
779,-738,-809
786,-495,387
815,-535,-749
930,560,-762
903,417,608
901,-447,338
-810,-659,629
-556,474,-738
-381,-644,-396
845,-624,-926
882,756,-734
-38,47,-5
-599,535,-679
942,781,-732

--- scanner 14 ---
415,-791,688
-423,-346,-630
890,652,453
111,-5,-108
888,592,458
866,563,575
471,-440,-707
-47,-31,63
-823,430,787
554,-808,753
599,-375,-749
624,-348,-724
-396,-438,-648
361,-760,723
-533,-641,725
-531,541,-866
-443,-336,-542
-787,524,790
-791,556,745
-689,-728,721
-586,630,-844
546,512,-569
-468,718,-860
438,562,-623
-528,-816,729
454,451,-562

--- scanner 15 ---
357,847,-506
431,735,445
392,-401,404
-416,-604,-802
-860,-246,637
633,789,457
-552,446,-529
-396,710,604
-479,-536,-686
318,-362,462
-131,49,-91
397,899,-391
-578,504,-608
641,-411,-375
-630,-226,588
33,86,13
414,907,-413
691,-575,-432
563,-504,-484
335,-472,434
605,643,441
-457,425,-510
-429,647,537
-427,-702,-765
-741,-281,720
-382,767,612

--- scanner 16 ---
-633,-674,333
-760,481,-460
668,819,527
550,-559,686
-659,644,592
680,680,-645
-369,-571,-867
695,694,589
-68,-97,-89
-687,-708,333
803,710,-763
-312,-712,-815
878,629,-690
-692,481,-371
418,-632,622
-463,-631,-811
796,-599,-558
84,61,-152
-603,-717,281
515,-578,637
-660,500,-537
558,720,539
849,-509,-497
-751,651,653
839,-474,-535
-767,590,542

--- scanner 17 ---
-635,-235,-728
856,593,-529
42,138,10
-628,796,-357
620,929,867
-642,-758,687
415,-579,694
524,-482,716
443,887,885
-812,609,859
536,814,933
463,-423,673
549,-399,-813
-688,-796,477
764,610,-682
-696,622,790
-546,-412,-698
770,-429,-825
-678,715,-449
-616,837,-485
-749,-352,-662
-746,613,718
-655,-765,498
-89,21,94
829,568,-554
745,-427,-765

--- scanner 18 ---
423,690,-627
465,808,-674
-681,-556,721
430,673,-675
-795,510,-788
-525,-433,-454
-797,681,-824
-463,-351,-573
-620,-614,805
885,-699,-394
528,450,736
-322,861,705
738,-488,424
-811,697,-757
464,358,724
-244,715,674
-743,-452,816
-484,-311,-360
814,-580,-456
174,59,-29
806,-589,-322
-391,772,589
52,30,126
771,-565,562
451,412,909
733,-432,552

--- scanner 19 ---
742,468,518
71,-35,-37
-710,627,527
361,-802,-714
383,484,-899
291,-811,-848
-819,-578,600
-390,280,-450
-743,-535,597
681,384,576
412,-794,-794
-672,637,394
-367,-422,-663
-823,-463,467
305,515,-753
-403,465,-396
672,-512,444
757,-600,483
757,-398,454
-323,-430,-682
-467,453,-488
-341,-622,-714
804,373,414
-677,765,450
452,600,-810

--- scanner 20 ---
-544,629,-427
20,-171,-93
371,-709,-476
-807,-622,-421
-806,-694,-491
-755,-685,-481
-944,497,344
-131,-48,-22
369,626,403
481,-697,-472
320,674,288
393,-780,491
-945,-503,648
453,366,-586
-767,586,374
-564,653,-501
438,-723,516
336,-701,341
249,553,298
437,359,-706
410,-789,-558
449,306,-624
-827,383,410
-762,647,-443
-937,-558,670
-853,-534,568

--- scanner 21 ---
-403,620,751
378,-908,681
483,-788,754
-544,-708,434
497,564,-657
596,-719,-595
-626,-720,487
-385,709,721
809,377,675
621,564,-506
368,-837,802
-474,684,-344
-396,-786,-581
-345,751,653
-392,-676,-569
-536,554,-353
421,-821,-573
-506,-704,494
-42,50,68
-592,768,-366
723,420,725
889,416,658
-478,-747,-661
571,-764,-652
78,-91,-29
591,547,-650

--- scanner 22 ---
-627,-326,675
675,671,-519
644,-874,521
-619,505,-796
-583,-508,-362
676,-892,329
582,751,-406
870,-748,-554
-477,-505,-353
908,-825,-381
-555,-412,656
-743,-359,606
-304,638,483
9,95,-66
-598,600,-727
625,807,505
962,-812,-566
-449,666,429
519,686,511
-724,642,-840
78,-93,-116
695,-848,321
543,859,578
551,787,-531
-563,-477,-461
-425,503,444

--- scanner 23 ---
449,677,-648
-173,-153,18
619,686,465
406,-769,-584
-657,687,422
619,618,376
600,586,542
-678,721,510
420,-754,650
471,-952,695
-566,241,-447
-402,-806,-446
-30,-38,107
-722,596,510
281,-742,-522
390,-877,768
-688,265,-536
-420,-783,-496
-740,-427,645
510,693,-537
-650,-424,811
-747,-398,662
-571,243,-507
380,-632,-534
456,585,-587
-450,-891,-397

--- scanner 24 ---
-590,-629,-597
710,304,866
154,-67,33
688,327,841
-288,796,-335
682,-762,-820
-485,-498,-550
517,559,-397
16,-171,107
626,-756,-590
-417,748,-451
710,-586,650
550,759,-382
663,-870,-675
670,663,-397
-665,-529,-537
-560,-462,847
-464,524,743
-637,-635,812
505,325,866
766,-481,778
-475,763,-295
-343,554,742
-559,-562,928
-363,493,860
700,-557,850

--- scanner 25 ---
-360,517,826
-696,281,-570
603,-630,-328
398,826,514
-72,20,78
-595,-629,-756
-679,373,-723
751,693,-833
-491,-582,706
781,741,-640
481,-719,-304
716,724,-779
365,-769,828
-534,-518,-740
335,-670,720
99,8,-67
317,-720,743
-721,308,-595
-475,601,892
-366,-644,679
486,-659,-306
-465,-632,569
321,790,439
-402,694,844
-541,-579,-648
372,686,539

--- scanner 26 ---
-644,-564,-581
-78,-46,-124
746,-537,581
679,-547,-661
341,710,-518
-514,-868,621
651,701,772
-691,362,751
807,-504,507
-636,583,-730
-539,692,-708
-719,558,799
-548,-868,567
-695,644,-687
665,-529,-630
690,718,688
-590,-579,-484
-663,-515,-616
-689,337,811
300,522,-459
837,-417,597
-505,-920,537
316,646,-490
737,-506,-663
624,552,672

--- scanner 27 ---
-485,-881,-896
-662,-839,-914
611,-700,578
149,-77,-47
922,661,522
-360,475,736
-690,627,-723
-335,477,796
870,652,606
-569,-766,-962
664,-776,-639
517,503,-631
661,-646,686
550,-707,670
-276,-520,525
-581,530,-745
-249,-545,500
581,576,-499
676,-875,-749
-642,471,-702
614,-750,-776
-341,-562,653
-31,-55,-158
-556,447,784
870,618,552
566,522,-596

--- scanner 28 ---
537,-616,-435
403,497,789
-734,976,683
363,-484,745
-758,-554,-727
-701,-382,724
419,605,-563
393,742,-513
378,-561,-443
631,-541,-453
-437,483,-376
-103,105,28
-808,-619,-628
-637,950,832
-389,568,-425
380,-348,803
433,620,671
-554,-468,688
-407,591,-475
-727,947,666
484,422,716
450,-304,752
294,632,-501
-809,-548,-770
-599,-363,804

--- scanner 29 ---
416,-491,827
-403,-379,-560
-420,-665,528
-735,829,-657
469,699,586
70,37,-13
474,-452,901
-363,-501,578
-379,809,822
-455,-322,-419
-401,817,805
576,664,-334
-773,839,-622
453,553,614
-459,-473,-500
619,505,-259
-712,786,-532
715,-713,-664
627,599,-366
-95,-46,95
-494,757,862
457,645,788
-472,-594,656
812,-716,-773
446,-562,884
750,-569,-730

--- scanner 30 ---
-647,-426,-938
-860,-733,501
574,-687,514
585,-616,682
-42,57,-114
-646,444,613
521,512,511
-628,360,-435
-787,378,617
429,562,430
-597,282,-581
834,849,-599
-718,494,604
752,-392,-623
557,574,389
651,-718,696
-851,-730,607
-512,429,-524
-832,-413,-884
751,-427,-488
-752,-761,587
865,833,-786
-17,-118,12
704,-385,-606
901,835,-615
-659,-490,-846

--- scanner 31 ---
836,-615,-464
-771,679,586
-424,-320,450
768,-856,848
549,430,-434
804,-557,-333
558,555,-331
532,596,-446
-605,-583,-478
18,16,-2
-510,869,-324
-522,697,-398
465,910,698
-584,-541,-690
-366,-465,370
-463,885,-363
442,769,719
-726,-509,-572
900,-504,-440
745,-834,885
873,-804,817
432,914,739
-425,-355,387
-811,658,799
-689,617,705

--- scanner 32 ---
-381,-530,548
-619,-781,-721
860,415,575
45,-33,-11
-40,124,57
-642,380,358
709,658,-482
552,761,-482
-643,560,-540
-689,419,342
632,627,-484
806,517,633
778,-429,-380
-630,454,530
480,-315,880
-341,-743,569
-647,479,-393
-729,-709,-800
838,598,572
-610,-647,-729
643,-428,-345
-463,-649,610
660,-476,-338
667,-346,896
-664,620,-466
651,-279,882

--- scanner 33 ---
-81,29,111
639,-815,-688
664,-497,366
-620,623,686
-407,651,-441
-383,610,-675
284,832,-400
-733,-571,888
574,770,671
-689,-396,-337
-783,-402,824
-651,-294,-463
521,-799,-600
-429,682,-517
576,-821,-511
711,-603,468
684,-585,376
-644,-458,-481
-780,-486,894
-548,705,686
418,766,758
316,790,-526
579,774,846
-525,576,739
320,835,-494

--- scanner 34 ---
709,-623,-800
618,-691,646
664,618,671
-542,704,-886
-941,-740,636
-655,-650,-435
680,-691,-803
-772,-679,-413
-937,770,332
-923,-690,613
-886,666,432
41,-133,-56
688,596,648
-893,-789,721
-860,806,407
-507,673,-940
705,618,606
750,-682,-769
672,772,-925
-109,-42,-127
588,-819,645
731,662,-942
-777,-739,-411
673,807,-862
-477,721,-747
743,-718,608

--- scanner 35 ---
437,680,-609
-66,-30,0
455,-616,-602
-601,-822,-631
-387,658,-849
345,-491,357
511,659,-669
453,-693,-571
364,663,-750
-559,646,-775
-852,788,661
407,348,463
-906,875,779
350,428,557
489,353,491
-510,-480,509
-875,697,692
390,-552,432
377,-591,394
-353,-410,470
-434,-474,366
453,-536,-468
-662,-790,-712
-604,-844,-814
-371,705,-810

--- scanner 36 ---
-426,485,-390
-615,-449,-336
-148,-62,57
-378,356,-274
614,664,523
547,837,547
604,-896,648
469,-434,-551
620,785,629
519,-381,-739
-620,411,803
642,616,-300
-681,431,921
-650,532,804
14,-121,179
-849,-437,927
681,481,-421
450,-348,-623
-870,-642,927
45,66,33
804,-870,699
-847,-620,902
796,-914,680
-411,543,-234
696,644,-445
-511,-602,-336
-415,-482,-321";
