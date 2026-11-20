#[derive(Clone,Debug)]
enum Exercise30State{Idle,Running,Finished}

struct Exercise30Record{id:usize,label:String,score:i32,state:Exercise30State}

impl Exercise30Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_30_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+30)).sum()
}

fn main(){
    println!("Exercise 30: bincode");
    let records=[
        Exercise30Record{id:1,label:"Ada".into(),score:98,state:Exercise30State::Finished},
        Exercise30Record{id:2,label:"Linus".into(),score:86,state:Exercise30State::Running},
        Exercise30Record{id:3,label:"Grace".into(),score:73,state:Exercise30State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_30_operation(&[12,7,19,3,25]));
}

const EXERCISE_30_27:i32=30;
const EXERCISE_30_29:i32=30;
const EXERCISE_30_30:i32=30;
const EXERCISE_30_31:i32=30;
const EXERCISE_30_32:i32=30;
const EXERCISE_30_33:i32=30;
const EXERCISE_30_34:i32=30;
const EXERCISE_30_35:i32=30;
const EXERCISE_30_36:i32=30;
const EXERCISE_30_37:i32=30;
const EXERCISE_30_38:i32=30;
const EXERCISE_30_39:i32=30;
const EXERCISE_30_40:i32=30;
const EXERCISE_30_41:i32=30;
const EXERCISE_30_42:i32=30;
const EXERCISE_30_43:i32=30;
const EXERCISE_30_44:i32=30;
const EXERCISE_30_45:i32=30;
const EXERCISE_30_46:i32=30;
const EXERCISE_30_47:i32=30;
const EXERCISE_30_48:i32=30;
const EXERCISE_30_49:i32=30;
const EXERCISE_30_50:i32=30;
const EXERCISE_30_51:i32=30;
const EXERCISE_30_52:i32=30;
const EXERCISE_30_53:i32=30;
const EXERCISE_30_54:i32=30;
const EXERCISE_30_55:i32=30;
const EXERCISE_30_56:i32=30;
const EXERCISE_30_57:i32=30;
const EXERCISE_30_58:i32=30;
const EXERCISE_30_59:i32=30;
const EXERCISE_30_60:i32=30;
const EXERCISE_30_61:i32=30;
const EXERCISE_30_62:i32=30;
const EXERCISE_30_63:i32=30;
const EXERCISE_30_64:i32=30;
const EXERCISE_30_65:i32=30;
const EXERCISE_30_66:i32=30;
const EXERCISE_30_67:i32=30;
const EXERCISE_30_68:i32=30;
const EXERCISE_30_69:i32=30;
const EXERCISE_30_70:i32=30;
const EXERCISE_30_71:i32=30;
const EXERCISE_30_72:i32=30;
const EXERCISE_30_73:i32=30;
const EXERCISE_30_74:i32=30;
const EXERCISE_30_75:i32=30;
const EXERCISE_30_76:i32=30;
const EXERCISE_30_77:i32=30;
const EXERCISE_30_78:i32=30;
const EXERCISE_30_79:i32=30;
