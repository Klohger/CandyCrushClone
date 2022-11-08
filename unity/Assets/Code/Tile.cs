using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Newtonsoft.Json;



public class Stage
{
    public Tile[,] tiles;
    public Item[] items;


    public static Stage testStage = new(new Tile[2, 2] { { new Tile((0, 1), null, 0), new Tile((1, 1)) }, { new Tile(null, (0, 0)), new Tile(null, (1, 0)) } }, new Item[1] {new Item((0,0))});

    public Stage(Tile[,] tiles, Item[] items)
    {
        this.tiles = tiles;
        this.items = items;
    }
}



public class StageScript : MonoBehaviour
{
    Stage stage;
    public Sprite[] masks;

    [SerializeField] private ulong stageID;
    private SpriteMask mask;
    private void Start() 
    {
        /*tiles = new Tile[2, 2]
        {
            { new Tile((0,1), null), new Tile((1,1),null), },
            { new Tile(null,(0,0)), new Tile(null,(1,0)), },
        };
        */
        //tiles = JsonConvert.DeserializeObject<Tile[,]>(System.IO.File.ReadAllText($"/data/stages/{stageID}.stg"));
        //mask.sprite = masks[stageID];
        instance = this; 
    }
    public static StageScript instance;
    
}

public class Tile
{
    public ulong? item;
    public (ulong, ulong)? prev;
    public(ulong, ulong)? next;

    public Tile((ulong, ulong)? next = null, (ulong, ulong)? prev = null, ulong? item = null)
    {
        this.prev = prev;
        this.next = next;
        this.item = item;
    }
}
