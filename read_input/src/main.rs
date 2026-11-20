#[derive(Clone,Debug)]
enum Exercise17State{Idle,Running,Finished}

struct Exercise17Record{id:usize,label:String,score:i32,state:Exercise17State}

impl Exercise17Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_17_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+17)).sum()
}

fn main(){
    println!("Exercise 17: read_input");
    let records=[
        Exercise17Record{id:1,label:"Ada".into(),score:98,state:Exercise17State::Finished},
        Exercise17Record{id:2,label:"Linus".into(),score:86,state:Exercise17State::Running},
        Exercise17Record{id:3,label:"Grace".into(),score:73,state:Exercise17State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_17_operation(&[12,7,19,3,25]));
}

const EXERCISE_17_27:i32=17;
const EXERCISE_17_29:i32=17;
const EXERCISE_17_30:i32=17;
const EXERCISE_17_31:i32=17;
const EXERCISE_17_32:i32=17;
const EXERCISE_17_33:i32=17;
const EXERCISE_17_34:i32=17;
const EXERCISE_17_35:i32=17;
const EXERCISE_17_36:i32=17;
const EXERCISE_17_37:i32=17;
const EXERCISE_17_38:i32=17;
const EXERCISE_17_39:i32=17;
const EXERCISE_17_40:i32=17;
const EXERCISE_17_41:i32=17;
const EXERCISE_17_42:i32=17;
const EXERCISE_17_43:i32=17;
const EXERCISE_17_44:i32=17;
const EXERCISE_17_45:i32=17;
const EXERCISE_17_46:i32=17;
const EXERCISE_17_47:i32=17;
const EXERCISE_17_48:i32=17;
const EXERCISE_17_49:i32=17;
const EXERCISE_17_50:i32=17;
const EXERCISE_17_51:i32=17;
const EXERCISE_17_52:i32=17;
const EXERCISE_17_53:i32=17;
const EXERCISE_17_54:i32=17;
const EXERCISE_17_55:i32=17;
const EXERCISE_17_56:i32=17;
const EXERCISE_17_57:i32=17;
const EXERCISE_17_58:i32=17;
const EXERCISE_17_59:i32=17;
const EXERCISE_17_60:i32=17;
const EXERCISE_17_61:i32=17;
const EXERCISE_17_62:i32=17;
const EXERCISE_17_63:i32=17;
const EXERCISE_17_64:i32=17;
const EXERCISE_17_65:i32=17;
const EXERCISE_17_66:i32=17;
const EXERCISE_17_67:i32=17;
const EXERCISE_17_68:i32=17;
const EXERCISE_17_69:i32=17;
const EXERCISE_17_70:i32=17;
const EXERCISE_17_71:i32=17;
const EXERCISE_17_72:i32=17;
const EXERCISE_17_73:i32=17;
const EXERCISE_17_74:i32=17;
const EXERCISE_17_75:i32=17;
const EXERCISE_17_76:i32=17;
const EXERCISE_17_77:i32=17;
const EXERCISE_17_78:i32=17;
const EXERCISE_17_79:i32=17;
