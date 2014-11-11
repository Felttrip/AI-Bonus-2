package aima.core.environment.wumpusworld;

/**
 * Created by ecgprc on 11/10/14.
 */
public class WumpusEnv {

    private  HybridWumpusAgent agent;
    private  WumpusKnowledgeBase kb;

    static public int[][][] map = new int[4][4][3];

    public WumpusEnv(){
        agent = new HybridWumpusAgent();
        kb = agent.kb;
    }

    public void init_map( String config ){

        String [] cellConfig = config.split(",");

        for( int i = 0; i <= 3; ++i ){
            for( int j = 0; j <=3; ++j ){
               String cur =  cellConfig[(4 * i) + j];
               for( int q = 0; q < cur.length(); ++q ){
                   char cur_c = cur.charAt(q);
                   switch(cur_c){
                       case 'S':
                           map[i][j][0] = 1;
                       break;
                       case 'B':
                           map[i][j][1] = 1;
                       break;
                       case 'G':
                           map[i][j][2] = 1;
                       break;
                   }
               }
            }
        }
    }

    public HybridWumpusAgent getAgent(){
        return agent;
    }


}
