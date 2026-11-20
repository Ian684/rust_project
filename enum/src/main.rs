#[derive(Clone,Debug)]
enum Exercise33State{Idle,Running,Finished}

struct Exercise33Record{id:usize,label:String,score:i32,state:Exercise33State}

impl Exercise33Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_33_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+33)).sum()
}

fn main(){
    println!("Exercise 33: enum");
    let records=[
        Exercise33Record{id:1,label:"Ada".into(),score:98,state:Exercise33State::Finished},
        Exercise33Record{id:2,label:"Linus".into(),score:86,state:Exercise33State::Running},
        Exercise33Record{id:3,label:"Grace".into(),score:73,state:Exercise33State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_33_operation(&[12,7,19,3,25]));
}

const EXERCISE_33_27:i32=33;
const EXERCISE_33_29:i32=33;
const EXERCISE_33_30:i32=33;
const EXERCISE_33_31:i32=33;
const EXERCISE_33_32:i32=33;
const EXERCISE_33_33:i32=33;
const EXERCISE_33_34:i32=33;
const EXERCISE_33_35:i32=33;
const EXERCISE_33_36:i32=33;
const EXERCISE_33_37:i32=33;
const EXERCISE_33_38:i32=33;
const EXERCISE_33_39:i32=33;
const EXERCISE_33_40:i32=33;
const EXERCISE_33_41:i32=33;
const EXERCISE_33_42:i32=33;
const EXERCISE_33_43:i32=33;
const EXERCISE_33_44:i32=33;
const EXERCISE_33_45:i32=33;
const EXERCISE_33_46:i32=33;
const EXERCISE_33_47:i32=33;
const EXERCISE_33_48:i32=33;
const EXERCISE_33_49:i32=33;
const EXERCISE_33_50:i32=33;
const EXERCISE_33_51:i32=33;
const EXERCISE_33_52:i32=33;
const EXERCISE_33_53:i32=33;
const EXERCISE_33_54:i32=33;
const EXERCISE_33_55:i32=33;
const EXERCISE_33_56:i32=33;
const EXERCISE_33_57:i32=33;
const EXERCISE_33_58:i32=33;
const EXERCISE_33_59:i32=33;
const EXERCISE_33_60:i32=33;
const EXERCISE_33_61:i32=33;
const EXERCISE_33_62:i32=33;
const EXERCISE_33_63:i32=33;
const EXERCISE_33_64:i32=33;
const EXERCISE_33_65:i32=33;
const EXERCISE_33_66:i32=33;
const EXERCISE_33_67:i32=33;
const EXERCISE_33_68:i32=33;
const EXERCISE_33_69:i32=33;
const EXERCISE_33_70:i32=33;
const EXERCISE_33_71:i32=33;
const EXERCISE_33_72:i32=33;
const EXERCISE_33_73:i32=33;
const EXERCISE_33_74:i32=33;
const EXERCISE_33_75:i32=33;
const EXERCISE_33_76:i32=33;
const EXERCISE_33_77:i32=33;
const EXERCISE_33_78:i32=33;
const EXERCISE_33_79:i32=33;
