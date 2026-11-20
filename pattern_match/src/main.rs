#[derive(Clone,Debug)]
enum Exercise15State{Idle,Running,Finished}

struct Exercise15Record{id:usize,label:String,score:i32,state:Exercise15State}

impl Exercise15Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_15_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+15)).sum()
}

fn main(){
    println!("Exercise 15: pattern_match");
    let records=[
        Exercise15Record{id:1,label:"Ada".into(),score:98,state:Exercise15State::Finished},
        Exercise15Record{id:2,label:"Linus".into(),score:86,state:Exercise15State::Running},
        Exercise15Record{id:3,label:"Grace".into(),score:73,state:Exercise15State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_15_operation(&[12,7,19,3,25]));
}

const EXERCISE_15_27:i32=15;
const EXERCISE_15_29:i32=15;
const EXERCISE_15_30:i32=15;
const EXERCISE_15_31:i32=15;
const EXERCISE_15_32:i32=15;
const EXERCISE_15_33:i32=15;
const EXERCISE_15_34:i32=15;
const EXERCISE_15_35:i32=15;
const EXERCISE_15_36:i32=15;
const EXERCISE_15_37:i32=15;
const EXERCISE_15_38:i32=15;
const EXERCISE_15_39:i32=15;
const EXERCISE_15_40:i32=15;
const EXERCISE_15_41:i32=15;
const EXERCISE_15_42:i32=15;
const EXERCISE_15_43:i32=15;
const EXERCISE_15_44:i32=15;
const EXERCISE_15_45:i32=15;
const EXERCISE_15_46:i32=15;
const EXERCISE_15_47:i32=15;
const EXERCISE_15_48:i32=15;
const EXERCISE_15_49:i32=15;
const EXERCISE_15_50:i32=15;
const EXERCISE_15_51:i32=15;
const EXERCISE_15_52:i32=15;
const EXERCISE_15_53:i32=15;
const EXERCISE_15_54:i32=15;
const EXERCISE_15_55:i32=15;
const EXERCISE_15_56:i32=15;
const EXERCISE_15_57:i32=15;
const EXERCISE_15_58:i32=15;
const EXERCISE_15_59:i32=15;
const EXERCISE_15_60:i32=15;
const EXERCISE_15_61:i32=15;
const EXERCISE_15_62:i32=15;
const EXERCISE_15_63:i32=15;
const EXERCISE_15_64:i32=15;
const EXERCISE_15_65:i32=15;
const EXERCISE_15_66:i32=15;
const EXERCISE_15_67:i32=15;
const EXERCISE_15_68:i32=15;
const EXERCISE_15_69:i32=15;
const EXERCISE_15_70:i32=15;
const EXERCISE_15_71:i32=15;
const EXERCISE_15_72:i32=15;
const EXERCISE_15_73:i32=15;
const EXERCISE_15_74:i32=15;
const EXERCISE_15_75:i32=15;
const EXERCISE_15_76:i32=15;
const EXERCISE_15_77:i32=15;
const EXERCISE_15_78:i32=15;
const EXERCISE_15_79:i32=15;
