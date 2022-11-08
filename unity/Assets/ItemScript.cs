using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Item
{
    public bool canMove;
    public (ulong, ulong)? tile;
    public Item((ulong, ulong)? tile)
    {
        this.tile = tile;
    }
}
