#[derive(Clone,Debug)]
enum Exercise24State{Idle,Running,Finished}

struct Exercise24Record{id:usize,label:String,score:i32,state:Exercise24State}

impl Exercise24Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_24_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+24)).sum()
}

fn main(){
    println!("Exercise 24: threads");
    let records=[
        Exercise24Record{id:1,label:"Ada".into(),score:98,state:Exercise24State::Finished},
        Exercise24Record{id:2,label:"Linus".into(),score:86,state:Exercise24State::Running},
        Exercise24Record{id:3,label:"Grace".into(),score:73,state:Exercise24State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_24_operation(&[12,7,19,3,25]));
}

const EXERCISE_24_27:i32=24;
const EXERCISE_24_29:i32=24;
const EXERCISE_24_30:i32=24;
const EXERCISE_24_31:i32=24;
const EXERCISE_24_32:i32=24;
const EXERCISE_24_33:i32=24;
const EXERCISE_24_34:i32=24;
const EXERCISE_24_35:i32=24;
const EXERCISE_24_36:i32=24;
const EXERCISE_24_37:i32=24;
const EXERCISE_24_38:i32=24;
const EXERCISE_24_39:i32=24;
const EXERCISE_24_40:i32=24;
const EXERCISE_24_41:i32=24;
const EXERCISE_24_42:i32=24;
const EXERCISE_24_43:i32=24;
const EXERCISE_24_44:i32=24;
const EXERCISE_24_45:i32=24;
const EXERCISE_24_46:i32=24;
const EXERCISE_24_47:i32=24;
const EXERCISE_24_48:i32=24;
const EXERCISE_24_49:i32=24;
const EXERCISE_24_50:i32=24;
const EXERCISE_24_51:i32=24;
const EXERCISE_24_52:i32=24;
const EXERCISE_24_53:i32=24;
const EXERCISE_24_54:i32=24;
const EXERCISE_24_55:i32=24;
const EXERCISE_24_56:i32=24;
const EXERCISE_24_57:i32=24;
const EXERCISE_24_58:i32=24;
const EXERCISE_24_59:i32=24;
const EXERCISE_24_60:i32=24;
const EXERCISE_24_61:i32=24;
const EXERCISE_24_62:i32=24;
const EXERCISE_24_63:i32=24;
const EXERCISE_24_64:i32=24;
const EXERCISE_24_65:i32=24;
const EXERCISE_24_66:i32=24;
const EXERCISE_24_67:i32=24;
const EXERCISE_24_68:i32=24;
const EXERCISE_24_69:i32=24;
const EXERCISE_24_70:i32=24;
const EXERCISE_24_71:i32=24;
const EXERCISE_24_72:i32=24;
const EXERCISE_24_73:i32=24;
const EXERCISE_24_74:i32=24;
const EXERCISE_24_75:i32=24;
const EXERCISE_24_76:i32=24;
const EXERCISE_24_77:i32=24;
const EXERCISE_24_78:i32=24;
const EXERCISE_24_79:i32=24;
